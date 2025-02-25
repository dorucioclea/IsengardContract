import logging
import os
import os.path
import pathlib
import shutil
import subprocess
import sys
import json
from argparse import ArgumentParser

logger = logging.getLogger("installer")

MIN_REQUIRED_PYTHON_MAJOR_VERSION = 3
MIN_REQUIRED_PYTHON_MINOR_VERSION = 6
MIN_REQUIRED_PYTHON_MINOR_VERSION_MACOS = 8

elrondsdk_path = None
exact_version = None


def main():
    global elrondsdk_path
    global exact_version

    parser = ArgumentParser()
    parser.add_argument("--modify-path", dest="modify_path", action="store_true", help="whether to modify $PATH (in profile file)")
    parser.add_argument("--no-modify-path", dest="modify_path", action="store_false", help="whether to modify $PATH (in profile file)")
    parser.add_argument("--elrondsdk-path", default=get_elrond_sdk_path_default(), help="where to install elrond-sdk")
    parser.add_argument("--exact-version", help="the exact version of erdpy to install")
    parser.set_defaults(modify_path=True)
    args = parser.parse_args()

    elrondsdk_path = os.path.expanduser(args.elrondsdk_path)
    modify_path = args.modify_path
    exact_version = args.exact_version

    logging.basicConfig(level=logging.DEBUG)

    operating_system = get_operating_system()
    python_major_version = sys.version_info.major
    python_minor_version = sys.version_info.minor

    logger.info("Checking user.")
    if os.getuid() == 0:
        raise InstallError("You should not install erdpy as root.")

    logger.info("Checking Python version.")
    logger.info(f"Python version: {sys.version_info}")
    if python_major_version < MIN_REQUIRED_PYTHON_MAJOR_VERSION or (python_major_version >= MIN_REQUIRED_PYTHON_MAJOR_VERSION and python_minor_version < MIN_REQUIRED_PYTHON_MINOR_VERSION):
        raise InstallError("You need Python 3.6 or later.")
    if operating_system == "osx" and python_minor_version < MIN_REQUIRED_PYTHON_MINOR_VERSION_MACOS:
        raise InstallError("On MacOS, you need Python 3.8 or later.")

    logger.info("Checking operating system.")
    logger.info(f"Operating system: {operating_system}")
    if operating_system != "linux" and operating_system != "osx":
        raise InstallError("Your operating system is not supported yet.")

    remove_installation()
    create_venv()
    install_erdpy()
    if modify_path:
        add_sdk_to_path()
        logger.info("""
###############################################################################
Upon restarting the user session, [$ erdpy] command should be available in your shell.
Furthermore, after restarting the user session, you can use [$ source erdpy-activate] to activate the Python virtual environment containing erdpy.
###############################################################################
""")


def get_operating_system():
    aliases = {
        "linux": "linux",
        "linux1": "linux",
        "linux2": "linux",
        "darwin": "osx",
        "win32": "windows",
        "cygwin": "windows",
        "msys": "windows"
    }

    operating_system = aliases.get(sys.platform)
    if operating_system is None:
        raise InstallError(f"Unknown platform: {sys.platform}")

    return operating_system


def remove_installation():
    old_folder = os.path.expanduser("~/ElrondSCTools")
    if os.path.isdir(old_folder):
        answer = input(f"Older installation in {old_folder} has to be removed. Allow? (y/n)")
        if answer.lower() not in ["y", "yes"]:
            raise InstallError("Installation will not continue.")
        shutil.rmtree(old_folder)
        logger.info("Removed previous installation (ElrondSCTools).")

    folder = get_erdpy_path()
    if os.path.isdir(folder):
        shutil.rmtree(folder)
        logger.info("Removed previous installation (virtual environment).")


def create_venv():
    require_venv()
    folder = get_erdpy_path()
    ensure_folder(folder)

    logger.info(f"Creating virtual environment in: {folder}.")
    import venv
    builder = venv.EnvBuilder(with_pip=True)
    builder.clear_directory(folder)
    builder.create(folder)

    # Create symlink to "bin/activate"
    link_path = os.path.join(elrondsdk_path, "erdpy-activate")
    if os.path.exists(link_path):
        os.remove(link_path)
    os.symlink(os.path.join(folder, "bin", "activate"), link_path)
    logger.info(f"Virtual environment has been created in: {folder}.")


def require_venv():
    operating_system = get_operating_system()

    try:
        import ensurepip
        import venv
        logger.info(f"Packages found: {ensurepip}, {venv}.")
    except ModuleNotFoundError:
        if operating_system == "linux":
            logger.info("Package [venv] or [ensurepip] not found, will be installed.")
            logger.info("Running [$ sudo apt-get install python3-venv]:")
            return_code = os.system("sudo apt-get install python3-venv")
            if return_code == 0:
                logger.info("Done installing [python3-venv].")
            else:
                raise InstallError("Packages [venv] or [ensurepip] not installed correctly.")
        else:
            raise InstallError("Packages [venv] or [ensurepip] not found, please install them first. See https://docs.python.org/3/tutorial/venv.html.")


def get_erdpy_path():
    return os.path.join(elrondsdk_path, "erdpy-venv")


def get_elrond_sdk_path_default():
    return os.path.expanduser("~/elrondsdk")


def ensure_folder(folder):
    pathlib.Path(folder).mkdir(parents=True, exist_ok=True)


def install_erdpy():
    logger.info("Installing erdpy in virtual environment...")
    erpy_versioned = "erdpy" if not exact_version else f"erdpy=={exact_version}"
    return_code = run_in_venv(["python3", "-m", "pip", "install", "--upgrade", "pip"])
    if return_code != 0:
        raise InstallError("Could not upgrade pip.")
    return_code = run_in_venv(["pip3", "install", "--no-cache-dir", erpy_versioned])
    if return_code != 0:
        raise InstallError("Could not install erdpy.")
    return_code = run_in_venv(["erdpy", "--version"])
    if return_code != 0:
        raise InstallError("Could not install erdpy.")

    logger.info("Checking and upgrading configuration file")
    upgrade_erdpy_config()

    # Create symlink to "bin/erdpy"
    link_path = os.path.join(elrondsdk_path, "erdpy")
    if os.path.exists(link_path):
        os.remove(link_path)
    os.symlink(os.path.join(get_erdpy_path(), "bin", "erdpy"), link_path)
    logger.info("You have successfully installed erdpy.")


def run_in_venv(args):
    if "PYTHONHOME" in os.environ:
        del os.environ["PYTHONHOME"]

    process = subprocess.Popen(args, env={
        "PATH": os.path.join(get_erdpy_path(), "bin") + ":" + os.environ["PATH"],
        "VIRTUAL_ENV": get_erdpy_path()
    })

    return process.wait()


def add_sdk_to_path():
    logger.info("Checking PATH variable.")
    PATH = os.environ["PATH"]
    if elrondsdk_path in PATH:
        logger.info(f"elrond-sdk path ({elrondsdk_path}) already in $PATH variable.")
        return

    profile_file = get_profile_file()
    logger.info(f"Adding elrond-sdk path [{elrondsdk_path}] to $PATH variable.")
    logger.info(f"[{profile_file}] will be modified.")

    with open(profile_file, "a") as file:
        file.write(f'\nexport PATH="{elrondsdk_path}:$PATH"\t# elrond-sdk\n')

    logger.info(f"""
###############################################################################
[{profile_file}] has been modified.
Please RESTART THE USER SESSION.
###############################################################################
""")


def get_profile_file():
    operating_system = get_operating_system()
    file = None

    if operating_system == "linux":
        file = "~/.profile"
    else:
        value = input("""Please choose your preferred shell:
1) zsh
2) bash
""")
        if value not in ["1", "2"]:
            raise InstallError("Invalid choice.")

        value = int(value)
        if value == 1:
            file = "~/.zshrc"
        else:
            file = "~/.bash_profile"

    return os.path.expanduser(file)


def upgrade_erdpy_config():
    config_path = os.path.expanduser("~/elrondsdk/erdpy.json")

    if not os.path.exists(config_path):
        return

    with open(config_path) as f:
        data = json.load(f)

    if "active" in data:
        return

    new_data = {
        "active": "default",
        "configurations": {
            "default": data
        }
    }

    with open(config_path, "w") as f:
        json.dump(new_data, f, indent=4)


class InstallError(Exception):
    inner = None

    def __init__(self, message, inner=None):
        super().__init__(message)
        self.inner = inner


if __name__ == "__main__":
    try:
        main()
    except Exception as err:
        logger.fatal(err)
        sys.exit(1)

    logger.info("""

For more information go to https://docs.elrond.com.
For support, please contact us at https://t.me/ElrondDevelopers.
""")

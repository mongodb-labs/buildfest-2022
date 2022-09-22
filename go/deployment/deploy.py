import os
import sys

from fabric import Connection
from jinja2 import Environment, FileSystemLoader

deployment_dir = os.path.dirname(os.path.realpath(__file__))
jinja_env = Environment(
    loader=FileSystemLoader(deployment_dir),
)

git_url = "git@github.com:mongodb-labs/buildfest-2022.git"
branch = "main-go"
temp_branch = "TEMP23847123648917263784"
service_file = "/etc/systemd/system/mongo.service"

user = "ubuntu"
working_directory = "/home/ubuntu"
repo_main_directory = "buildfest-2022"
repo_go_directory = "go"

server_ip = "54.157.211.168"


def upload_service_file(connection: Connection):
    """
    Upload the service file to the VPS instance.

    Parameters:
        connection: the connection to the given vps.
    """
    print("Creating service file")
    tmp_service_location = f"{working_directory}/tmp_service_file"

    service_template = jinja_env.get_template("mongo.service.j2").render(
        {
            "working_directory": working_directory,
            "repo_main_directory": repo_main_directory,
            "repo_go_directory": repo_go_directory,
            "user": user,
        }
    )

    local_tmp_file = os.path.join(deployment_dir, "mongo.service")
    with open(local_tmp_file, "x") as file:
        file.write(service_template)

    connection.put(local=local_tmp_file, remote=tmp_service_location)
    os.remove(local_tmp_file)

    connection.sudo(f"mv {tmp_service_location} {service_file}")
    connection.sudo("systemctl daemon-reload")
    connection.sudo("systemctl enable mongo.service")


def restart_service(connection: Connection):
    """
    Restarts the service and outputs the status.

    Parameters:
        connection: the connection to the given vps.

    """
    print("Restarting service")
    connection.sudo("systemctl restart mongo.service")
    connection.sudo("systemctl status mongo.service")


def pull_repository(connection: Connection):
    """
    Update to the most recent version of your desired branch.
    This includes loads of clean up to make sure it does not fail:
    clean, reset, re-checkout branch (in case of rebase or whatever), etc.

    Parameters:
        connection: the connection to the given vps.

    """
    print("Updating repository.")

    connection.run("git fetch --prune --all")
    connection.run("git reset --hard")
    if connection.run(f"git checkout -b {temp_branch}", warn=True).failed:
        connection.run(f"git checkout {temp_branch}", warn=True)
    connection.run(f"git branch -D {branch}", warn=True)
    connection.run(f"git checkout {branch}")
    connection.run(f"git branch -D {temp_branch}")
    connection.run("git clean -xfd")
    connection.run(f"git pull origin {branch}")


def main():
    if len(sys.argv) < 2:
        print("Please provide the path to the private key file:")
        print("python3 deploy.py ~/.ssh/buildfest-2022-go.pem")
        return

    print("Starting deployment.")

    pkey = sys.argv[1]
    connection = Connection(host=server_ip, user="ubuntu", port=22, connect_kwargs={"key_filename": [f"{pkey}"]})

    # If necessary, docker will be installed.
    # We HAVE to do this here, since `connection.sudo` does not work with `with connection.cd`.
    connection.sudo(f"snap install docker")

    with connection.cd(working_directory):
        # Clean up before starting a new deployment.
        connection.run(f"rm -rf {repo_main_directory}")

        # Avoid fingerprinting questions
        connection.run("ssh-keygen -F github.com || ssh-keyscan github.com >> ~/.ssh/known_hosts")
        connection.run(f"ssh-keygen -F {server_ip} || ssh-keyscan {server_ip} >> ~/.ssh/known_hosts")

        # Clone the repository if this is the first run.
        if connection.run(f"test -d {repo_main_directory}", warn=True).failed:
            print("Cloning repository.")
            connection.run(f"git clone {git_url} {repo_main_directory}")

        with connection.cd(repo_main_directory):
            pull_repository(connection)

            with connection.cd(repo_go_directory):
                connection.run(f"mv docker-compose.yml.example docker-compose.yml")

    upload_service_file(connection)

    restart_service(connection)

    print("Deployment successfully finished.")


if __name__ == "__main__":
    main()

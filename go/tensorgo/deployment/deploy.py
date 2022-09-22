import os
import sys

from fabric import Connection
from pathlib import Path
from jinja2 import Environment, FileSystemLoader

application_name = "tensorgo"
docker_compose_example_file_url = "../docker-compose.yml.example"
docker_compose_file_url = "../docker-compose.yml"
service_file_name = f"{application_name}.service"
mongo_uri = 'mongodb+srv://demo:password1234@examples.mx9pd.mongodb.net/?retryWrites=true&w=majority'

local_deployment_directory: Path = Path()
local_deployment_directory: Path = local_deployment_directory.resolve()
local_application_directory: Path = local_deployment_directory.parent
local_application_parent_directory: Path = local_application_directory.parent
local_jinja_environment: Environment = Environment(
    loader=FileSystemLoader(local_deployment_directory),
)

server_ip = "54.157.211.168"
server_user = "ubuntu"
server_home_directory = f"/home/{server_user}"
server_application_directory = f"{server_home_directory}/{application_name}"
server_service_folder = "/etc/systemd/system"
server_service_file_url = f"{server_service_folder}/{service_file_name}"


def read_private_key_file_url_from_sys_args() -> str:
    if len(sys.argv) < 2:
        print("Please provide the path to the private key file. Example:")
        print("python3 deploy.py /home/dominic/.ssh/buildfest-2022-go.pem")
        sys.exit(0)

    private_key_file_url = sys.argv[1]

    return private_key_file_url


def create_connection_from_sys_args(private_key_file_url: str) -> Connection:
    print("\nCreating connection to the server.")

    connection = Connection(host=server_ip, user=server_user, port=22,
                            connect_kwargs={"key_filename": [f"{private_key_file_url}"]})

    return connection


def create_docker_compose_file():
    print("\nCreating docker-compose file.")

    # Create a copy of the example file.
    os.system(f"cp {docker_compose_example_file_url} {docker_compose_file_url}")

    # Read the contents of the file in read mode.
    docker_compose_file = open(docker_compose_file_url, "rt")
    file_content = docker_compose_file.read()

    # Replace the Mongo URI
    file_content = file_content.replace('MONGODB_URI_HERE', mongo_uri)

    # Open the file again in write mode.
    docker_compose_file.close()
    docker_compose_file = open(docker_compose_file_url, "wt")

    # Write the contents with the replaced uri.
    docker_compose_file.write(file_content)
    docker_compose_file.close()


def install_docker_on_remote(connection: Connection):
    print("\nInstalling docker on the server.")

    connection.sudo(f"snap install docker")


def upload_application(connection: Connection):
    print(f"\nUploading {application_name}.")

    compressed_file_name = f"{application_name}.tar.gz"
    compressed_file_url = f"{local_application_parent_directory}/{compressed_file_name}"

    os.system(f"cd {local_application_parent_directory} && tar -vzcf {compressed_file_name} {application_name}")
    connection.put(local=compressed_file_url, remote=server_home_directory)
    os.remove(compressed_file_url)
    with connection.cd(server_home_directory):
        connection.run(f"tar -vzxf {compressed_file_name}")


def upload_service_file_and_restart_service(connection: Connection):
    print(f"\nCreating and uploading {service_file_name}.")

    service_template = local_jinja_environment.get_template(f"{application_name}.service.j2").render(
        {
            "server_application_directory": server_application_directory,
            "server_user": server_user,
        }
    )

    local_service_file = os.path.join(local_deployment_directory, service_file_name)
    with open(local_service_file, "x") as file:
        file.write(service_template)

    connection.put(local=local_service_file, remote=server_home_directory)
    os.remove(local_service_file)

    connection.sudo(f"systemctl stop {service_file_name}")
    connection.sudo(f"mv {server_home_directory}/{service_file_name} {server_service_folder}")

    print(f"\nRestarting {service_file_name}.")
    connection.sudo("systemctl daemon-reload")
    connection.sudo(f"systemctl enable {service_file_name}")
    connection.sudo(f"systemctl start {service_file_name}")
    connection.sudo(f"systemctl status {service_file_name}")


def main():
    private_key_file_url: str = read_private_key_file_url_from_sys_args()

    connection: Connection = create_connection_from_sys_args(private_key_file_url)

    create_docker_compose_file()

    install_docker_on_remote(connection)

    upload_application(connection)

    upload_service_file_and_restart_service(connection)

    print(f"\nDeployment of {application_name} successfully finished.")


if __name__ == '__main__':
    main()

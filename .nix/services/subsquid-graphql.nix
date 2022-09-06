{ database, pkgs, graphqlPort, ...}:
{
  service = {
    build.context = "${../../subsquid}";
    depends_on = {
      "${database.name}" = {
        condition = "service_healthy";
      };
    
    };
    ports = [
      "${toString graphqlPort}:4000"
    ];
    environment = {
      DB_NAME = database.name; 
      DB_HOST = database.host;
      DB_PASS = database.password;
      DB_PORT_PG = database.port;
    };
  };

  # Unfortunately, arion does not model this field yet.
  # We can add a PR that adds it right below this option 
  # https://github.com/hercules-ci/arion/blob/e5fb978143240f8d293e6e5acc9691acf472928d/src/nix/modules/service/docker-compose-service.nix#L66  
  out.service.build.dockerfile = "graphql.Dockerfile";
}

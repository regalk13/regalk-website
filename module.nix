self: { lib, pkgs, config, ... }:
let
  cfg = config.services.regalk;
  sitePkg = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
in
{
  options.services.regalk = {
    enable = lib.mkEnableOption "Regalk website service";
    address = lib.mkOption {
      type = lib.types.str;
      default = "0.0.0.0:3000";
      description = "Server listen address";
    };
  }; # options to activate the service and address

  config = lib.mkIf cfg.enable {

    systemd.services.regalk = {
      description = "Regalk website service";
      after = [ "network.target" ]; # ensure netwrok is up
      wantedBy = [ "multi-user.target" ]; # multi-user setup
      serviceConfig = {
        ExecStart = "${sitePkg}/bin/regalk"; # bin
        Environment = [
          "LEPTOS_SITE_ROOT=${sitePkg}/site"
          "LEPTOS_SITE_ADDR=${cfg.address}"
          "BLOGS_DIR=${sitePkg}/blogs"
        ]; # env the app uses
        Restart = "always";
        PrivateTmp = true;
      };
    }; # default service regalk that will run the leptos page
  };
}
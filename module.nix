self: { lib, pkgs, config, … }:
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
  };

  config = lib.mkIf cfg.enable {

    systemd.services.regalk = {
      description = "Regalk website service";
      after = [ "network.target" ];
      wantedBy = [ "multi-user.target" ];
      serviceConfig = {
        ExecStart = "${sitePkg}/bin/regalk";
        Environment = [
          "LEPTOS_SITE_ROOT=${sitePkg}/site"
          "LEPTOS_SITE_ADDR=${cfg.address}"
        ];
        Restart = "always";
        PrivateTmp = true;
      };
    };
  };
}
# NixOS Module Options

This document details the configuration options available for the Configuratarr NixOS module.

## services.configuratarr.enable

Whether to enable Configuratarr declarative configuration service.



*Type:*
boolean



*Default:*

```nix
false
```



*Example:*

```nix
true
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.package



The configuratarr package to use.



*Type:*
package



*Default:*

```nix
<derivation configuratarr-0.1.0>
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.prune



Pass ` --prune ` to delete server-side resources absent from the config.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings



Configuration written verbatim to ` configuratarr.yaml `.

Each top-level attribute is an instance you name, and its ` type ` selects
the service (e.g. ` radarr-v3 `); add more attributes to manage more apps.
See ` docs/radarr-v3-config.md ` (and the equivalent per-service docs) for
the available fields of each resource.



*Type:*
attribute set



*Default:*

```nix
{ }
```



*Example:*

```nix
{
  my-radarr = {
    type = "radarr-v3";
    url = "http://localhost:7878";
    api_key = "\${env.RADARR_API_KEY}";
    tags = [ { label = "managed"; } ];
  };
}

```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.waitForHealthy



Pass ` --wait-for-healthy ` to poll each service’s health endpoint before syncing.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.waitTimeout



Max seconds to wait per service when ` waitForHealthy ` is enabled.



*Type:*
unsigned integer, meaning >=0



*Default:*

```nix
120
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



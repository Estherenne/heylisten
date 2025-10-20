# heylisten
an alternative to `poweralertd`

## why not poweralertd?
I made this purely for myself because `poweralertd` was sending
7 notifications for the one event, and also because I was bored.

# installation
`heylisten` is available as a crate:
```
$ cargo install heylisten
```

for Nix and NixOS users, a flake is also available:
```nix
{
  inputs = {
    # ...

    heylisten.url = "github:Estherenne/heylisten";
  };

  outputs = {
    # ...
    heylisten,
    # ...
  } @ inputs: {
    nixosConfigurations."..." = nixpkgs.lib.nixosSystem {
      # ...

      specialArgs = {
        inherit inputs;
      };

      # if you want to install in home-manager:
      modules = [
        # ...

        home-manager.nixosModules.home-manager
        {
          home-manager.extraSpecialArgs = {
            inherit inputs;
          };

          # ...
        }
      ];
    };
  };
}
```

you may also install imperatively (not recommended):
```
$ nix profile install github:Estherenne/heylisten
```

## what's that name about?
navi from OOT often says "Hey, listen!"

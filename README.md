# miniscript-tabconf
miniscript-tabconf

pyo3.rs

# Deploy axum web api on ubuntu

- Installs Nix
  
```
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
```

- Fetches cachix
  
```
nix-env -iA cachix -f https://cachix.org/api/v1/install
```

- Allows the OS to run devenv

```
echo "trusted-users = root ubuntu" | sudo tee -a /etc/nix/nix.conf && sudo pkill nix-daemon
```

  
```
cachix use devenv
```

- Fetches devenv

```
nix-env -if https://github.com/cachix/devenv/tarball/latest
```

  

```
git clone some_repo
```

```
cd some_repo
```

- Enables dev shell (bundles, cargo, tools and some other things)
  
```
devenv shell
```

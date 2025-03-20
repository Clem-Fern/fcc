# FCC - Flat Configuration Compliance

FCC is a small tool allowing you to check configuration compliance.
Carefull, it is a personal project. However I would be glad to merge PR if anyone want to improve it.

### Quick start - CLI

```
curl -L -o /usr/local/bin/fcc "https://github.com/Clem-Fern/fcc/releases/latest/download/fcc"
chmod u+x /usr/local/bin/fcc
fcc -h
```

### Quick start - Ansible

Download `fcc_check_compliance` and place it in your ansible workspace library folder.
```
# ansible.conf
...
# (pathspec) Colon separated paths in which Ansible will search for Modules.
library=/home/debian/.ansible/plugins/modules:/usr/share/ansible/plugins/modules
```

Ex. :
```
curl -L -o /home/debian/.ansible/plugins/modules/fcc_check_compliance "https://github.com/Clem-Fern/fcc/releases/latest/download/fcc_check_compliance"
chmod u+x /home/debian/.ansible/plugins/modules/fcc_check_compliance
```

Ansible tasks ex. :
```yaml
---
- name: Gather only the config and default facts
  cisco.ios.ios_facts:
    gather_subset:
      - config

- name: Test policy against configuration
  fcc_check_compliance:
    return: changed #value: failed or changed
    configuration: {{ ansible_net_config }}
    policy: |
        hostname TEST

        #[state=absent]
        enable password cisco  

```

### Use in your cargo project

```toml
[dependencies]
fcc = { git = "https://github.com/Clem-Fern/fcc", features = ["serde"] }
```

### Build
```
docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust cargo build --release
```
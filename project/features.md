# Features

## Beta command line

```bash

rpass -> default help

help - prints help
list - list all password stored
init - initializes the password database (triggered with the first use of the command, if no database)
add - stores a new password
delete - deletes a password
dump - print a password to std out (protected by master password)
generate - generates a new password
```

## Features à dev pendant le codelab
- `list` avec comfy-table pour le print
- `add` avec inquire pour les input + validation strong password avec zxcvbn
- `delete`
- `dump`
- `generate` avec générateur via passwords

## Features à préparer
- initialiser le repo cargo
- ajouter clap,serde,logger,anyhow
- configurer clap pour la CLI (serde)
- poser les modules :
    - cli_parser: `parse(cli {}) // parse la cli via CLAP`
    - dispatcher `dispatch(cli {]}) // dispatch les actions de la CLI`
    - database `init(), get_all(), get_single(s), add(new_p {}), delete(s) // interface base fichier json`
    - password `hash(s, s), validate_is_strong(s) // gestion des mots de passe`
    - console_io `write(s), writeln(s), title(s), table([headers], [data]), ask(type, s) // se charge des I/O consoles`

## Evols possible après codelab
- connexion avec HaveIBeenPwnd pour lister les mdp dans les leaks
- gérer plusieurs bases de mdp
- rempalcer zxcvbn par passablewords

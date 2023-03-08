# Préparer son PC/Mac pour l'atelier

Pour préparer votre participation à l'atelier, rien de plus simple : il suffit d'installer Rust sur votre machine et préparer votre environnement de développement.

*Durée : 5min*

## Installer Rust

### Pour Windows

Sous Windows, rendez-vous sur [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) et suivez les instructions pour installer Rust. À un certain moment de l'installation, vous recevrez un message expliquant que vous aurez également besoin des outils `MSVC build tools for Visual Studio 2013`.

Pour acquérir ces outils, vous devez installer [Visual Studio 2022](https://visualstudio.microsoft.com/downloads/). Lorsqu'on vous demande quelles workloads installer, incluez :

* `Desktop Development with C++`
* `SDK Windows 10 ou 11`
* Le pack de langue anglais, ainsi que tout autre pack de langue de votre choix

> Pour vérifier votre installation, ouvrez un invite de commande et tapez `rustc --version`.

### Pour Linux & MacOS

Si vous utilisez Linux ou macOS, ouvrez un terminal et entrez la commande suivante :

```bash
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

La commande télécharge un script et lance l'installation de l'outil `rustup`, qui installe la dernière version stable de Rust. Il se peut que l'on vous demande votre mot de passe. Si l'installation est réussie, la ligne suivante apparaîtra :

```text
Rust is installed now. Great!
```

Vous aurez également besoin d'un `linker`, qui est un programme que Rust utilise pour joindre ses programmes compilés en un seul fichier. 

Il est probable que vous en ayez déjà un. 

Si vous obtenez des erreurs d'édition de liens, vous devriez installer un compilateur C, qui inclut généralement un éditeur de liens. Un compilateur C est également utile car certains paquets Rust courants dépendent du code C et auront besoin d'un compilateur C.

Sur macOS, vous pouvez obtenir un compilateur C en exécutant :

```bash
xcode-select --install
```

Les utilisateurs de Linux doivent généralement installer `GCC` ou `Clang`, selon la documentation de leur distribution. Par exemple, si vous utilisez Ubuntu, vous pouvez installer le paquet `build-essential`.

> Pour vérifier votre installation, ouvrez un terminal et tapez `rustc --version`.

### Si ces méthodes ne vous conviennent pas

Les méthodes précédentes conviendront à 99.9% d'entre vous. Si vous êtes sur un OS très spécifique, vous pouvez suivre la documentation suivante qui détaille toutes les manières d'installer Rust : [https://forge.rust-lang.org/infra/other-installation-methods.html](https://forge.rust-lang.org/infra/other-installation-methods.html).

Vous pouvez aussi trouver la documentation complête d'installation ici : [https://doc.rust-lang.org/book/ch01-01-installation.html](https://doc.rust-lang.org/book/ch01-01-installation.html)

## Préparer son environnement de développement

Pour développer en Rust, l'IDE de choix est VSCode. Nous allons donc installer et configurer VSCode pour Rust.

> **Note :** La configuration proposé n'affectera pas vos configurations déjà existantes pour d'autres langages.

> **Note :** Vous pouvez parfaitement venir à l'atelier avec votre propre IDE de configuré pour Rust, cet article détaille seulement une manière parmi d'autre de se setup un environnement de développement pour Rust.

### Installer VSCode

Pour ceux qui ne l'auraient pas déjà, la première étape est d'installer VSCode.

La page officiel de VSCode propose directement toutes ses versions au téléchargement ; vous pouvez donc vous y rendre pour installer cet IDE : [https://code.visualstudio.com/download](https://code.visualstudio.com/download).

### Configurer VSCode

Maintenant que nous avons VSCode d'installé, il faut y ajouter quelques modules supplémentaires pour que l'expérience de développement Rust y soit optimale.

Vous pouvez utiliser l'onglet `Extensions` *(quatrième onglet de la barre latérale gauche par défaut)* et rechercher et installer les extensions suivantes :

* `Rust Analyzer` 🡆 Code completion, imports, goto, références, documentation au survol, etc.
* `Crates` 🡆 Auto-complétion des fichiers Cargo depuis la base de paquets crates.io.
* `Code LLDB` 🡆 Breakpoints avancés, intégration du débugger au code, etc.
* `Better TOML` 🡆 Synthax highlight et auto-complétion des fichiers TOML.

> Optionnellement, je vous recommande aussi l'extension `Error lens` qui permet d'afficher les erreurs non pas en soulignant de rouge, mais en les expliquant en bout de ligne de code. 
>
> C'est une extension particulièrement pratique, mais qui impactera tout vos autres projets VSCode.

### Configurer Clippy

Clippy est un linter plus poussé que le linter de base de Rust `cargo check`. Il possède une collection de 550 lints qui, en plus de vous aider à débugger votre code, vous permettront de développer en Rust plus proprement et efficacement.

Pour l'utiliser, vous devez déjà l'ajouter à votre toolchain Rust. Pour cela, dans un terminal (windows/linux/mac) :

```bash
rustup component add clippy
```

En suite, vous devez spécifier à VSCode d'utiliser Clippy au lieu du linter par défaut, pour cela :

1. Allez dans `file` > `preference` > `settings`.
2. Recherchez l'entrée `Rust analyzer › Check Command`
3. Remplacez la valeur par `clippy`
4. Vérifiez bien que la valeur `Rust analyzer › Check On Save: Enable` est bien cochée.
5. Quittez la fenêtre de settings.

Voilà, maintenant Clippy vous accompagnera pendant vos développements, et tout votre environnement est prêt pour l'atelier.

Rendez-vous le 23 Mars !

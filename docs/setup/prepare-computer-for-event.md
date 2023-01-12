## Préparer son PC/Mac pour l'atelier

Pour préparer votre participation à l'atelier, rien de plus simple : il suffit d'installer Rust sur votre machine.

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

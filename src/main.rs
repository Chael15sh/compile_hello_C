use std::fs::File;
use std::io::Write;

fn main() {
    // Créer le fichier C "hello.c"
    let mut file = File::create("hello.c").expect("Impossible de créer le fichier hello.c");
    let c_code = r#"
    #include <stdio.h>

    int main() {
        printf("Hello, World!\n");
        return 0;
    }
    "#;
    file.write_all(c_code.as_bytes()).expect("Impossible d'écrire dans le fichier");

    println!("Le fichier hello.c a été créé avec succès !");
}

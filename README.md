# Word Count in Rust
## Description

This Rust program demonstrates how to count the occurrences of each unique word in a list of words. It makes use of the Rust standard library's HashMap to efficiently count each word.

### Importation des bibliothèques
```rust
use std::collections::HashMap;
```
Nous importons `HashMap` de la bibliothèque standard de Rust, ce qui nous permet de stocker des paires clé-valeur.

### Définition d'une structure `Word`

```rust
#[derive(Debug)]
struct Word {
    word: String,
    count: u32,
}
```
Nous définissons une structure `Word` qui a deux champs: `word` (un `String`) pour stocker le mot lui-même et `count` (un `u32`) pour stocker le nombre d'occurrences de ce mot.

### Implémentation d'une fonction `new` pour `Word`

```rust
impl Word {
    fn new(word: String) -> Word {
        Word { word, count: 1 }
    }
}
```
Ici, nous avons une fonction `new` qui prend un `String` comme argument et retourne une nouvelle instance de `Word` avec le `count` initialisé à 1.

### Fonction principale `main`

```rust
fn main() {
    let mut word_list: HashMap<String, Word> = HashMap::new();
    let input_words = vec!["apple", "banana", "apple", "orange", "banana", "apple"];
```
Nous initialisons une `HashMap` appelée `word_list` qui stockera chaque mot unique comme clé et une instance de `Word` comme valeur. Nous avons également une liste `input_words` contenant les mots que nous voulons compter.

```rust
    for word in input_words.iter() {
        if let Some(entry) = word_list.get_mut(&word.to_string()) {
            entry.count += 1;
        } else {
            word_list.insert(word.to_string(), Word::new(word.to_string()));
        }
    }
```
Nous parcourons chaque mot dans `input_words`. Si le mot est déjà dans `word_list`, nous augmentons son compteur `count`. Sinon, nous insérons une nouvelle entrée dans `word_list` avec `count` initialisé à 1.

```rust
    for (_, word) in &word_list {
        println!("Word: {}, Count: {}", word.word, word.count);
    }
}
```
Enfin, nous affichons le mot et son nombre d'occurrences.

J'espère que cela clarifie comment le code fonctionne. N'hésitez pas à poser des questions supplémentaires !
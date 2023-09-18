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

Voici les morceaux de code que j'ai ajoutés pour exporter les résultats dans un fichier CSV, ainsi que des explications pour chaque partie.

### Ajout de la bibliothèque `csv`

```rust
use csv::Writer;
```

Ici, nous importons `Writer` depuis la bibliothèque `csv`, ce qui nous permettra de créer et d'écrire dans des fichiers CSV.

### Initialisation du Writer CSV et écriture des en-têtes

```rust
let mut wtr = Writer::from_path("word_count.csv").expect("Cannot create file");
wtr.write_record(&["Word", "Count"]).expect("CSV write error");
```

1. `Writer::from_path("word_count.csv")` crée un nouveau fichier CSV appelé `word_count.csv`.
2. `expect("Cannot create file")` génère une erreur si le fichier ne peut pas être créé.
3. `wtr.write_record(&["Word", "Count"])` écrit les en-têtes "Word" et "Count" dans la première ligne du fichier CSV.

### Écriture des données dans le fichier CSV

```rust
for (_, word) in &word_list {
    wtr.write_record(&[word.word.as_str(), word.count.to_string().as_str()])
        .expect("CSV write error");
}
```

Dans cette boucle, pour chaque `word` dans `word_list`, nous utilisons `wtr.write_record` pour écrire le mot et son compteur dans le fichier CSV.

### Flush du Writer CSV

```rust
wtr.flush().expect("CSV write error");
```

La méthode `flush()` assure que toutes les données en attente sont écrites dans le fichier. Ceci est important pour s'assurer que le fichier CSV est correctement finalisé.

Ces ajouts permettent d'exporter les résultats dans un fichier CSV que vous pouvez ensuite ouvrir avec n'importe quel logiciel qui supporte ce format, comme Microsoft Excel ou LibreOffice Calc.
# Demo 1 - double

## Init

En premier lieux, créer le projet rust:

```
cargo new double
```

Cargo.toml:

```
[workspace]
members = [
    "double",
]
```

Dans les dépendances du projet, rajouter wyzoid:

```
wyzoid = "0.1.2"
```

## Shader

Avant de créer notre premier shader, on va lancer notre compiler de shader:

```
shdrr -v
```

`shdrr` va compiler automatiquement nos fichiers GLSL en SPIR-V, qui est un bytecode standard de shader supporté par Vulkan, OpenGL et OpenCL.

On peut maintenant commencer notre shader. On doit d'abord préciser la version de GLSL à utiliser:

```
#version 450
```

On utilise la version 450 car c'est la version minimum pour être compatible avec vulkan.
On va ensuite définir un espace d'éxécution pour notre shader en 3 dimensions:

```
[snippet:layout]
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
```

J'éxpliquerais un peu plus tard à quoi il correspond, mais en mettant toute nos dimensions à 1, on définit que notre shader s'éxécute unitairement.

La deuxième chose c'est de donner la définition de notre buffer, c'est à dire un espace mémoire dans la carte graphique dans laquelle on a mis nos donnée:

```
[snippet:buffer]
layout(std430, set = 0, binding = 0) buffer Data { float data[]; };
```

On accèder à plusieurs buffer dans un shader, donc on doit définir lequel on veut via un numéro de set et de binding. Ici on indique qu'on veut celui attacher au set 0 et au binding 0.

On écrit enfin le coeur de notre fonction:

```
void main() {
  uint i = gl_GlobalInvocationID.x;
  data[i] *= 2.0;
}
```

La première chose à faire est de récupèrer l'identifiant unique d'éxécution du shader, qui est l'équivalent de notre variable d'itération `i`, `n` ... si on était séquentiel.

On récupère notre donnée associé et on la multiplie par deux.

Voila, notre shader est prêt.

[vérifier que shdrr a correctement compilé]

## Partie client

On peut maintenant faire le code rust qui va s'occuper d'envoyer nos donnée et notre shader sur le gpu, executé le tout et récupèrer la sortie.

En premier lieux, on va indiquer que l'on utilise wyzoid, générer un tableau de nombre aléatoire [0,1] et créer le chemin vers notre shader:

```
extern crate wyzoid;

const VALUES: usize = 64;

[...]
let input = wyzoid::utils::rand_vec::<f32>(VALUES, 0.0, 1.0);
let shader = std::path::PathBuf::from("double.cs.spirv");
```

Ensuite, on va utiliser notre API pour définir ce que l'on veut éxécuter sur notre GPU:

```
let job = wyzoid::high::job::JobBuilder::new()
        .add_buffer(&input, 0, 0)
        .add_shader(&shader)
        .add_dispatch((VALUES as u32, 1, 1))
        .build();
```

On définit que l'on veut mettre nos donnée dans le buffer qui est dans le set 0, binding 0, comme indiqué dans notre shader.
On passe notre shader et enfin, on défini comment on éxécute notre shader.

On a définis plus tôt que on éxécutait notre shader unitairement, c'est à dire, on veut que 1 exécution = 1 donnée.
Donc, ici, on veut que notre dimention x (car c'est celle que l'on a utiliser pour l'identifiant) soit éxactement égale au nombre de donnée à traiter.

On éxécute ensuite notre shader et on affiche le temps que l'on a mis:

```
let (res, timing) = job.execute();
println!("gpu exe: {}", timing);
```

A noté que l'éxécution est non-bloquante, mais mon petit framework est bloquant pour simplifier les choses.

Ensuite, on veut quand même vérifier que nos données de sorties sont correcte, et on peut en profiter pour comparer la vitesse du CPU et du GPU:

```
[snippet:copyvec]
let s = Instant::now();
let mut test: Vec<f32> = Vec::with_capacity(input.len());
for v in &input {
    test.push(double(v.clone()));
}
println!("cpu exe: {}ms", wyzoid::utils::get_fract_s(s.elapsed()));

[snippet:compvec]
for i in 0..input.len() {
    if res[0][i] != test[i] {
        println!("ERROR");
    }
}
```

Voila, on peut exécuter notre premier programme GPGPU:

```
cargo run -p double
```

... Et on constate que l'on est beaucoup moins rapide que le CPU.
Alors que faire.

La première chose, si on éxécute une deuxième fois, on vois que le temps "shader" est beaucoup moins long.
En fait, le driver de notre GPU doit re-compiler notre shader en quelque chose que le GPU comprend.
La plus part des driver vont mettre en cache ce résultat et on économise donc du temps.
Qui plus est, on n'a qu'a faire la compilation et l'initialization que une fois dans notre application.
Donc a moins de faire des one-shot comme dans cette démo, on devrais pouvoir ignorer ce temps.

On fait un aller retour entre RAM et VRAM, pour 64 donnée ce n'est pas la peine.

```
const VALUES: usize = 1024 * 1024;
```

Enfin, on sous utilise largement notre GPU puisque l'on est entrain d'éxécuter unitairement notre shader.

Retournous sur le layout dont on parlais plus tôt et disons que l'on s'éxécute sur 8 données à la fois:

```
layout(local_size_x = 8, local_size_y = 1, local_size_z = 1) in;
```

Sur notre code client, il faut donc diviser notre nombre d'éxécution par 8:

```
.add_dispatch((VALUES as u32 / 8, 1, 1))
```

On voit que l'on a gagné beaucoup de performance. Essayons maintenant avec 64, 128 voir 512 données.

On ne gagne aucune performance, voir on en perd, pourquoi ?

[retour slide]

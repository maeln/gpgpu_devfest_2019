# Demo 2 - sin

## Init

Pareille

```
cargo new taylor
```

```
[workspace]
members = [
    "double",
    "taylor",
]
```

et on copie-colle notre code précédent. On crée un shader `taylor.cs`.

## Le shader

En premier lieux, on va avoir besoin de définir quelques fonctions dont on a besoin:

```
[snippet:mpow]
float mpow(float a, int b) {
  float acc = 1.0;
  for (int i = 0; i < b; ++i) {
    acc *= a;
  }
  return acc;
}
```

On redéfinit une fonction d'exponentiation car celle de GLSL est indéfinit si a est inférieur a 0.
On a aussi besoin d'une fonction factoriel.

```
[snippet:fact]
float fact(float x) {
  float acc = 1.0;
  for (float i = 1.0; i <= x; i += 1.0) {
    acc = acc * i;
  }
  return acc;
}
```

On peut maintenant coder notre fonction taylor:

```
float taylor_sin(float a, int iter) {
  float acc = 0.0;
  for (int i = 0; i < iter; ++i) {
    float n = float(i);
    [snippet:taylor]
    acc += (mpow(-1.0, i) * pow(a, 2.0 * n + 1.0)) / fact(2.0 * n + 1.0);
  }
  return acc;
}
```

On peu désormais écrire notre fonction main:

```
void main() {
  uint idx = gl_GlobalInvocationID.x;
  float data = tosin[idx];
  tosin[idx] = taylor_sin(data, 32);
}
```

## Client

Il nous faut mettre à jour le liens vers notre shader:

```
let shader = std::path::PathBuf::from("taylor.cs.spirv");
```

On doit mettre a jour notre copie du tableau aussi:

```
for v in &input {
    test.push(f32::sin(v.clone()));
}
```

On doit aussi changer notre comparaison car on compare des flottants:

```
if !wyzoid::utils::f32_cmp(res[0][i], test[i], 0.001) {
    println!("ERROR");
}
```

On peut désormais lancer notre projet:

```
cargo run -p taylor
```

## Branching

Un ami mathematicien ma signaler que cette série de taylor convergeais plus rapidement si notre valeur d'entrée etait proche de 0.
Je vais donc essayer de faire du early-exit pour nous éviter des iterations inutiles.

```
#define BRANCH

...

#ifndef BRANCH
  tosin[idx] = taylor_sin(data, 32);
#else
  if(data < 0.2) {
    tosin[idx] = taylor_sin(data, 8);
  } else if (data < 0.5) {
    tosin[idx] = taylor_sin(data, 16);
  } else {
    tosin[idx] = taylor_sin(data, 32);
  }
#endif
```

On lance le projet et ... on gagne pas de performance ! Pire, on semble en perdre.
Que ce passe t'il ?

[retour slide]

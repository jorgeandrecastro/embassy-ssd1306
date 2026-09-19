# 📜 Changelog - embassy-ssd1306

L'objectif principal de ce driver est de rester ultra-léger et pragmatique. Nous évitons d'alourdir le binaire avec des fonctionnalités complexes, afin de préserver les ressources critiques des microcontrôleurs (RAM/Flash).


# Changelog

Toutes les modifications notables de `embassy-ssd1306` sont documentées ici.
Le projet suit le [versionnage sémantique](https://semver.org/lang/fr/).

## [0.7.0]

### Ajouté
- **34 nouveaux glyphes** (table portée de 60 à 94 entrées) :
  - ASCII : `*` `'` `;` `{` `}` `\` `~` `$` `` ` ``
  - Opérateurs mathématiques : `×` `÷` `±` `°` `≤` `≥` `≠` `≈` `√` `∞` `∫` `∂` `→`
  - Lettres grecques minuscules : `α` `β` `λ` `θ` `μ` `π` `σ` `φ` `ω`
  - Lettres grecques majuscules : `Δ` `Σ` `Ω`
- `font::char_to_glyph_unicode(char)` : conversion d'un caractère Unicode en index de glyphe
  (variantes acceptées : `ϑ`, `ϕ`, `µ`).
- `draw_text(x, page, &str)` : affichage de texte UTF-8 (ASCII + symboles + grec).
- Constantes de glyphes nommées dans `font` (`font::DOT`, `font::LAMBDA`, `font::THETA`…)
  à utiliser avec `draw_char` à la place des index numériques.
- `Ssd1306::release()` pour récupérer le bus I2C.

### Modifié
- **Réorganisation modulaire** (API publique inchangée, `Ssd1306` se déréférence vers `FrameBuffer`) :
  `command` (opcodes), `font`, `framebuffer`, `graphics`, `text`, `driver`.
- Les opcodes de `init()` sont désormais nommés (`command::SET_CONTRAST`, `command::CHARGE_PUMP`…).

### Corrigé
- Débordement `u8` possible dans `draw_hline`, `draw_vline`, `draw_bitmap` (dessin désormais rogné).
- `draw_rect` avec `w == 0` ou `h == 0` ne provoque plus de débordement.
- `draw_char` : un index de glyphe invalide est ignoré, et un glyphe proche du bord droit
  ne déborde plus sur la page suivante.

### Note
- Les majuscules grecques sans glyphe dédié (`Π`, `Λ`, `Θ`, `Φ`…) ne sont pas gérées pour l'instant.


**[0.6.0] - Symboles et formatage**
Nouveaux glyphes : Ajout de 5 nouveaux glyphes : `^`, `#`, `@`, `&`, `"`. Cela permet d'afficher des symboles de puissance/accent, des commentaires, des adresses email, des conjonctions et des chaînes de caractères.

Fonctionnalité : Support étendu pour les annotations, les métadonnées et les caractères spécialisés.



**[0.4.0] - Symboles opérateurs et formatage**
Symboles additionnels : Ajout de 5 nouveaux symboles : `+`, `/`, `|`, `_`. Cela permet d'afficher des équations, des séparateurs de contenu, des traits de soulignement, et des ondulations.

Fonctionnalité : Support complet pour les calculs mathématiques simples, le formatage structuré et les visualisations textuelles.

**[0.3.0] - Caractères spéciaux et sécurité**
Caractères additionnels : Ajout du support pour 14 nouveaux caractères spéciaux : `(`, `)`, `,`, `[`, `]`, `%`, `<`, `>`, `=`, `?`,`!`, `:`. Cela permet d'afficher des textes plus riches et informatifs (états, pourcentages, expressions mathématiques, etc.).


Documentation enrichie : Mise à jour des exemples pour illustrer l'utilisation des nouveaux caractères.


**[0.2.4] - Mise à jour de stabilité**
Flexibilité Embassy : Migration vers une plage de version embassy-time = ">=0.3, <0.5". Cela permet d'utiliser le driver sur Pico 2 (RP2350) et les versions futures d'Embassy sans conflits de compilation.


**[0.2.3] - Exemple matériel**
Validation Pico : Ajout d'un exemple complet, validé et testé sur Raspberry Pi Pico (RP2040). Idéal pour démarrer un projet rapidement avec un câblage standard.

**[0.2.1] - Ponctuation**
Ajout du caractère . : Support du point pour les affichages numériques ou de fin de phrase.

Design Choice : Un espace automatique est inséré après le point. C'est un choix volontaire pour garder un code simple, efficace et éviter des fonctions de calcul de largeur de caractères trop lourdes pour le processeur.

**[0.2.0] - Gestion du texte**
Affichage des lettres : Introduction du support alphabétique.

Pragmatisme : Par défaut, l'affichage utilise des majuscules. C'est un choix de simplicité pour garantir une lisibilité maximale sur les petits écrans OLED tout en minimisant la taille de la police en mémoire Flash.
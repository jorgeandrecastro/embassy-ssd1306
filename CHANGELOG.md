# 📜 Changelog - embassy-ssd1306

L'objectif principal de ce driver est de rester ultra-léger et pragmatique. Nous évitons d'alourdir le binaire avec des fonctionnalités complexes, afin de préserver les ressources critiques des microcontrôleurs (RAM/Flash).

**[0.3.0] - Caractères spéciaux et sécurité**
Caractères additionnels : Ajout du support pour 14 nouveaux caractères spéciaux : `(`, `)`, `,`, `[`, `]`, `%`, `<`, `>`, `=`, `?`, , `!`, `:`. Cela permet d'afficher des textes plus riches et informatifs (états, pourcentages, expressions mathématiques, etc.).

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
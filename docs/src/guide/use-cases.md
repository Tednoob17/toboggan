# Use Cases

Toboggan découple l'ordinateur qui fait tourner la présentation de l'appareil qui la contrôle. Résultat : des scénarios impossibles avec un setup classique deviennent triviaux.

## Deux présentateurs, une seule scène

Vous présentez à deux. Le premier est au podium, le second est de l'autre côté de la scène — ou carrément dans la salle. Pas question de faire passer le clicker devant tout le monde.

- Le serveur tourne sur le PC branché au projecteur.
- Un des présentateurs utilise le PC.
- L'autre ouvre le client Web sur son téléphone ou une tablette.
- Chacun peut avancer les slides depuis son propre appareil, où qu'il soit dans la salle.

Plus besoin de dire « tu peux cliquer pour moi ? » ou de se passer le boîtier discrètement.

## Pas de télécommande ? Votre téléphone suffit

La salle est équipée mais il n'y a pas de télécommande, ou la pile est morte. Toboggan transforme n'importe quel smartphone en remote :

- Lancez le serveur sur le PC relié au projecteur.
- Ouvrez le client Web sur votre téléphone.
- Promenez-vous sur scène — avancez, reculez, sautez à n'importe quelle slide depuis votre poche.

Pas de Bluetooth, pas de dongle, pas de matériel spécifique.

## Le câble vidéo est loin du podium

La prise HDMI / DisplayPort est au fond de la salle, mais vous voulez parler depuis le devant. Vous êtes obligé de rester collé au bureau du projecteur ? Non :

- Posez le PC près du projecteur (là où est le câble).
- Rejoignez le podium avec votre téléphone ou votre tablette.
- Contrôlez les slides sans fil depuis l'endroit où vous parlez vraiment.

## Présenter sans son PC

PC oublié, batterie à plat, ou ordinateur qui refuse de coopérer le jour J. Pas de panique :

- Lancez le serveur Toboggan sur le PC d'un collègue, un Raspberry Pi, ou même une VM dans le cloud.
- Ouvrez le client Web sur n'importe quel appareil — téléphone, tablette, prêt — et présentez.
- Les slides et les sessions terminal tournent sur le serveur, pas sur votre appareil.

Utile aussi pour les démos embarquées : vous pouvez lancer des sessions SSH depuis le serveur, exécuter des commandes en direct, et tout contrôler depuis votre téléphone.

## En salle de formation ou workshop

Le formateur avance les slides, et chaque participant suit sur son propre écran :

- Le serveur tourne sur la machine du formateur.
- Le formateur contrôle le rythme depuis son PC.
- Les participants ouvrent le client Web sur leur propre appareil (téléphone, tablette, PC) et voient la slide en cours.
- Tout le monde suit à son rythme sans être collé à l'écran du formateur.

## Accessibilité

Un présentateur à mobilité réduite peut contrôler la présentation depuis une position confortable (assis dans la salle, depuis un appareil fixé au fauteuil roulant, etc.), sans avoir à rester debout au bureau du projecteur.

## Session de questions / débogage en direct

Pendant les questions, le présentateur peut s'approcher du public tout en gardant la main sur les slides :

- Téléphone dans la poche, il avance les slides pour répondre aux questions.
- Il peut même revenir en arrière pour montrer un détail technique sans retourner au bureau.

## Terminaux embarqués dans les slides

Toboggan permet d'intégrer des sessions terminal live dans une slide. Utile pour :

- Montrer un fuzzer en cours d'exécution — le terminal se met à jour en temps réel.
- Faire une démo de debug sans basculer entre fenêtres.
- Laisser les participants distants interagir avec le même terminal depuis leur navigateur.

Le combo « contrôle des slides depuis n'importe quel appareil + terminaux embarqués » rend Toboggan particulièrement adapté aux conférences techniques, formations et présentations en binôme.

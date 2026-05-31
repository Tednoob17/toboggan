# Dépannage

## `toboggan-cli` ne trouve pas mes fichiers

Vérifiez que le dossier contient bien des fichiers `.md` et que les chemins sont corrects.

## La page Web ne se connecte pas

- Vérifiez que le serveur tourne bien (`toboggan-server`).
- Vérifiez que le port n'est pas bloqué par un pare-feu.
- Essayez `http://127.0.0.1:8080` au lieu de `localhost`.

## Les slides n'avancent pas

- Vérifiez la connexion WebSocket dans la console du navigateur.
- Redémarrez le serveur.

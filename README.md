Question 3 :
Sauver l’image obtenue au format png. Que se passe-t-il si l’image de départ avait un canal
alpha?
Expliquer dans le README de votre rendu ce qui se passe ici.

Question 3 / reponse : 

Lorsque l'image de départ a un canal alpha (transparence), ce canal est ignoré lors de la conversion en niveaux de gris et lors du seuillage binaire. La conversion en niveaux de gris ne conserve pas la transparence, et le seuillage binaire ne prend pas en compte le canal alpha. Lorsque l'image est convertie en RGB pour la sauvegarde, le canal alpha n'est pas réintroduit. Par conséquent, l'image finale sauvegardée au format PNG est en RGB sans transparence, même si l'image d'origine avait un canal alpha.

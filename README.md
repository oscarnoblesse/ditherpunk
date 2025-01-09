

Question2 :
    Pour ouvrir une image depuis un fichier, on utilise ... On obtient un DynamicImage, à quoi
    correspond ce type? Comment obtenir une image en mode rbg8 ...
    Une image arbitraire peut avoir des pixels de nature différente:
    • avec un nombre variables de canaux (couleurs ou non, transparence ou non)
    • avec un type de donnée différent pour les canaux (entiers sur un octet, flottants ou autres)
    Passer l’image d’entrée en mode rgb8, c’est-à-dire avec 3 canaux R, G, B, représentés chacun
    par un u8.

Question 2 Reponse : 
    Pour ouvrir une image depuis un fichier, on utilise image::open(&args.input)?. DynamicImage représente une image avec des propriétés variables telles que le nombre de canaux et les types de données. Pour obtenir une image en mode rgb8, on utilise img.to_rgb8()

Question 3 :
Sauver l’image obtenue au format png. Que se passe-t-il si l’image de départ avait un canal
alpha?
Expliquer dans le README de votre rendu ce qui se passe ici.

Question 3 / reponse : 

Lorsque l'image de départ a un canal alpha (transparence), ce canal est ignoré lors de la conversion en niveaux de gris et lors du seuillage binaire. La conversion en niveaux de gris ne conserve pas la transparence, et le seuillage binaire ne prend pas en compte le canal alpha. Lorsque l'image est convertie en RGB pour la sauvegarde, le canal alpha n'est pas réintroduit. Par conséquent, l'image finale sauvegardée au format PNG est en RGB sans transparence, même si l'image d'origine avait un canal alpha.

Question4 : 
    Afficher dans le terminal la couleur du pixel (32, 52) de l’image de votre choix.

Question4 reponse : 
    Pour recuperais un pixel de mon image j'ai utiliser ce code la premier ligne permet de convertire mon image en rgb8. Puis la deuxieme ligne permet de recuperais mon pixel donc cezs valeur RGB. 
    ![alt text](imagePourReadMe/image.png)
    ![alt text](imagePourReadMe/image-2.png)

Question5 :
    Passer un pixel sur deux d’une image en blanc. Est-ce que l’image obtenue est reconnaissable?

Question5 reponse : 
    oui l'image optenu est reconnaissable. Pour le faire il suffisais de parcourir tout les pixel est une fois que la somme de x et y etat modulo 2 alors le programme transformer le pixel en blanc. 
    ![alt text](imagePourReadMe/imageInitiale.jpeg)
    ![alt text](imagePourReadMe/imageQuestion5PixelBlanc.png)
    


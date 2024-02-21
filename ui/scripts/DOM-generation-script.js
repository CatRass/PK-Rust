export function generateBoxes() {
    let boxElement = document.getElementsByClassName("boxDisplay")[0];
    for (let box = 1; box < 12; box++) {
        boxElement.innerHTML += "<div class=\"pcBox\" id=\"box-" + box +"\" style=\"display: none;\"></div>";

        let currBox = document.getElementById("box-"+box);
        for (let row = 0; row < 4; row++) {
            currBox.innerHTML += "<div class=\"row-" + row + "\" style=\"width: calc(45vw);\">";

            let currRow = currBox.getElementsByClassName("row-" + row)[0];
            for (let column = 0; column < 5; column++) {
                currRow.innerHTML += "<img class=\"column-"+column+" boxCreature\" data-box=\""+box+"\" data-row=\""+row+"\" data-column=\""+column+"\"></img>";
            }
        }
    }
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
const { invoke } = window.__TAURI__.tauri

let jsonSave;

async function pkmnSpriteCorrection(pkmnName) {
    if (pkmnName == "nidoran♀") {
        return "nidoran-f"
    } else if (pkmnName == "nidoran♂"){
        return "nidoran-m"
    } else {
        return pkmnName
    };
}

async function loadSaveFile() {

    // A temporary save variable, so jsonSave doesn't get poisoned 
    // upon user closing the file select option
    let tempSave = JSON.parse(await invoke("getSaveFile"));
    console.log(tempSave);

    // If the user has not selected a save file, the JSON
    // data returned will just be of the id being -1
    if (tempSave.id == -1) {
        console.log("User didn't select a save file")
    } else {
        document.querySelector("#saveEditor").style.display = "none";
        jsonSave = tempSave;
        let trainerDiv = document.querySelector("#trainerDetails")

        trainerDiv.style.display = "block";

        window.trainerName.value = jsonSave.trainer;
        window.trainerID.value = jsonSave.id;
        window.trainerMoney.value = jsonSave.money;

        for (let i=0; i<6; i++) {
            let currElement = document.getElementsByClassName("creatureBox")[i];
            let currDetails = currElement.getElementsByClassName("creature")[0];
            currDetails.src = "assets/creature-sprites/blank.png";
        }

        // Display Current Party
        for(creature in jsonSave.party) {

            let currElement = document.getElementsByClassName("creatureBox")[creature];
            let currDetails = currElement.getElementsByClassName("creature")[0];

            let pkmnName = await pkmnSpriteCorrection(jsonSave.party[creature].species.name.toLowerCase());
  
            console.log("Currently loading: " + pkmnName);
            currDetails.src = "assets/creature-sprites/"+pkmnName+".png";

        }

        window.party.style.display = "flex";
    }

}

async function testSaveFile() {
    await invoke("printSaveFile");
}

async function displayCreature(id) {
    let currPokemon = jsonSave.party[id-1];
    let nameDisplay = document.querySelector("#name");
    let lvlDisplay = document.querySelector("#level");
    let movesDisplay = document.getElementById("moves");

    let pkmnImage = document.getElementById("displayPkmn");
    let pkmnName = await pkmnSpriteCorrection(jsonSave.party[id-1].species.name.toLowerCase());
    console.log("Currently loading: " + pkmnName);
    pkmnImage.src = "assets/creature-sprites/"+pkmnName+".png";

    nameDisplay.value = currPokemon.nickname;
    lvlDisplay.textContent = "LVL " + currPokemon.level;

    for (move in currPokemon.moves) {
        let moveId = Number(move)+1;
        let currMove = movesDisplay.querySelector("#move-"+moveId);

        currMove.innerHTML = currPokemon.moves[move].name;
    };

    document.querySelector("#saveEditor").style.display = "flex";
}

window.addEventListener("DOMContentLoaded", () => {
    document.querySelector("#saveSelector").addEventListener("submit", (e) => {
        e.preventDefault();
        loadSaveFile();
    });

    document.querySelector("#testSave").addEventListener("submit", (e) => {
        e.preventDefault();
        testSaveFile();
    });

    let creatureButtons = document.querySelectorAll(".creatureSelect");
    creatureButtons.forEach((creature) => {
        creature.addEventListener("click", (e) => {
            e.preventDefault();
            displayCreature(creature.id);;
        });
    });

});
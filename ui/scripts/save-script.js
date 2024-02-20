// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
const { invoke } = window.__TAURI__.tauri

let jsonSave;
let currentBox = 0;

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

        // Display Boxes
        for(box in jsonSave.pc) {
            let currentBoxElement = document.getElementById("box-"+box);
            currentBoxElement.style.display = "none";
            currentBox = 0;

            for(let i=0; i<4; i++){
                let currentRow = currentBoxElement.getElementsByClassName("row-"+i)[0];

                for(let j=0; j<5; j++){
                    let currentColumn = currentRow.getElementsByClassName("column-"+j)[0];
                    // currentColumn.textContent = "";
                    currentColumn.src = "assets/creature-sprites/blank.png";
                    if(jsonSave.pc[box][5*i+j] != null) {
                        // currentColumn.textContent = jsonSave.pc[box][5*i+j].nickname;
                        let pkmnName = await pkmnSpriteCorrection(jsonSave.pc[box][5*i+j].species.name.toLowerCase());
                        
                        console.log("Currently loading: " + pkmnName);
                        currentColumn.src = "assets/creature-sprites/"+pkmnName+".png";
                    }
                    
                }

            }
        }
        document.getElementById("boxes").style.display = "grid";
        document.getElementById("box-0").style.display = "grid";

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

        let moveName = currPokemon.moves[move].name;
        let moveTyping = currPokemon.moves[move].typing;

        if (moveName == "Null") {
            currMove.style.display = "none";
        } else {
            currMove.style.display = "block";
        }
        currMove.innerHTML = moveName;
        
        console.log(moveTyping);
        currMove.style.background = "var(--"+moveTyping+")";
    };

    document.querySelector("#saveEditor").style.display = "flex";
}

async function changeBox(id) {
    let currentBoxElement = document.getElementById("box-"+currentBox);

    if(id == "backBoxBtn" && currentBox > 0) {
        currentBoxElement.style.display = "none";
        currentBox -= 1;
        document.getElementById("box-"+currentBox).style.display = "grid";
    } else if (id == "frwdBoxBtn" && currentBox < 11) {
        currentBoxElement.style.display = "none";
        currentBox += 1;
        document.getElementById("box-"+currentBox).style.display = "grid";
    }
    
};

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

    let backBox = document.querySelectorAll(".boxBtn");
    backBox.forEach((button) => {
        button.addEventListener("click", (e) => {
            e.preventDefault();
            changeBox(button.id);;
        });
    });

});
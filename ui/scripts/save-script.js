// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
const { invoke } = window.__TAURI__.tauri

async function loadSaveFile() {
    let jsonSave = JSON.parse(await invoke("getSaveFile"));
    console.log(jsonSave);

    // If the user has not selected a save file, the JSON
    // data returned will just be of the id being -1
    if (jsonSave.id == -1) {
        console.log("User didn't select a save file")
    } else {
        let trainerDiv = document.querySelector("#trainerDetails")

        trainerDiv.style.display = "flex";

        window.trainerName.value = jsonSave.trainer;
        window.trainerID.value = jsonSave.id;
        window.trainerMoney.value = jsonSave.money;

        // Display Current Party
        for(creature in jsonSave.party){

            let currElement = document.getElementsByClassName("creatureBox")[creature];

            let currDetails = currElement.getElementsByClassName("details")[0];
            let pkmnName = jsonSave.party[creature].species.name.toLowerCase();
            console.log(pkmnName);
            currDetails.src = "assets/creature-sprites/"+pkmnName+".png";

        }

        window.party.style.display = "flex";
    }

}

async function testSaveFile() {
    await invoke("printSaveFile");
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
});
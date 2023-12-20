import { Player } from "./player";
import type { Round } from "./round";

export class Game {
    players: Array<Player> = [];
    rounds: Array<Round> = [];

    constructor(names: Array<string>) {
        this.players = new Array();
        names.forEach(name => {
           this.players.push(new Player(name)); 
        });
    }
}
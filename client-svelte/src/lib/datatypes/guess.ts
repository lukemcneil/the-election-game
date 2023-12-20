import { Player } from "./player";
import { Answer } from "./answer";

export class Guess {
    player: Player = new Player("");
    guesses: Array<Answer> = [];
}
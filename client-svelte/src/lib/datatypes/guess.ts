import { Player } from "./player";
import { Answer } from "./answer";

export class Guess {
    player: Player | undefined;
    guesses: Array<Answer> | undefined;
}
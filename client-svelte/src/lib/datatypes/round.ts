import { Player } from "./player"
import { Answer } from "./answer"
import { Guess } from "./guess"

export class Round {
    player: Player | undefined;
    answers: Array<Answer> | undefined;
    guesses: Array<Guess> | undefined;
}
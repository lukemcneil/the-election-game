import { Player } from "./player";
import type { Round } from "./round";

export class Game {
    players: Array<Player> | undefined;
    rounds: Array<Round> | undefined;
}
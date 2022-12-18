module Main exposing (main)

import Browser
import Debug as Debug
import Dict exposing (Dict)
import Html exposing (Html, button, div, h1, li, pre, text, ul)
import Html.Attributes exposing (class, id)
import Html.Events exposing (onClick)
import Http
import Json.Decode exposing (..)
import Json.Encode



-- MAIN


main =
    Browser.element
        { init = init
        , update = update
        , subscriptions = subscriptions
        , view = view
        }



-- MODEL


type Model
    = None
    | Failure
    | Loading
    | Success String
    | HasData GameState


init : () -> ( Model, Cmd Msg )
init _ =
    ( None
    , createGame
    )



-- UPDATE


type Msg
    = GetData
    | GotGameData (Result Http.Error GameState)
    | MadeGame (Result Http.Error {})


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GetData ->
            ( Loading, getGameData )

        GotGameData result ->
            case result of
                Ok data ->
                    ( HasData data, Cmd.none )

                Err _ ->
                    ( Failure, Cmd.none )

        MadeGame _ ->
            ( model, getGameData )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none



-- VIEW


view : Model -> Html Msg
view model =
    case model of
        Failure ->
            div []
                [ button [ onClick GetData, class "bg-blue-600 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded" ]
                    [ text "get game data" ]
                , pre [] [ text "I was unable to load your book." ]
                ]

        Loading ->
            text "Loading..."

        Success fullText ->
            div []
                [ button [ onClick GetData, class "bg-blue-600 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded" ]
                    [ text "get new game data" ]
                , pre [] [ text fullText ]
                ]

        None ->
            button [ onClick GetData, class "bg-blue-600 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded" ]
                [ text "get game data" ]

        HasData data ->
            div []
                [ button [ onClick GetData, class "bg-blue-600 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded" ]
                    [ text "get new game data" ]
                , h1 [ class "text-4xl" ] [ text "Players: " ]
                , ul []
                    (List.map (\l -> li [] [ text ("-" ++ l) ]) data.players)
                , h1 [ class "text-4xl" ] [ text "Rounds: " ]
                , ul []
                    (List.map
                        (\l ->
                            div []
                                [ pre [] [ text ("Question: " ++ l.question) ]
                                , pre [] [ text "Answers: " ]
                                , div [] (List.map (\l1 -> li [] [ text ("-" ++ l1.player ++ ": " ++ l1.answer) ]) l.answers)
                                , pre [] [ text "Guesses: " ]
                                , div []
                                    (List.map
                                        (\l2 ->
                                            div []
                                                [ li [] [ text ("-" ++ l2.player ++ "'s guesses") ]
                                                , div [] (List.map (\l3 -> li [] [ text ("----" ++ l3.player ++ ": " ++ l3.answer) ]) l2.answers)
                                                ]
                                        )
                                        l.guesses
                                    )
                                , div [] [ text "--------------------------------------------" ]
                                ]
                        )
                        data.rounds
                    )
                ]


type alias Round =
    { question : String
    , answers : List Answer
    , guesses : List Guess
    }


type alias Answer =
    { player : String
    , answer : String
    }


type alias Guess =
    { player : String
    , answers : List Answer
    }


type alias GameState =
    { players : List String
    , rounds : List Round
    }


createGame : Cmd Msg
createGame =
    Http.request
        { method = "PUT"
        , headers = []
        , url = "http://localhost:8172/api/v1/game/5"
        , body = Http.jsonBody (Json.Encode.object [ ( "player", Json.Encode.string "player1" ) ])
        , expect = Http.expectJson MadeGame (succeed {})
        , timeout = Nothing
        , tracker = Nothing
        }


getGameData : Cmd Msg
getGameData =
    Http.get
        { url = "http://localhost:8172/api/v1/game/5"
        , expect = Http.expectJson GotGameData decoder
        }


decoder : Decoder GameState
decoder =
    map2
        GameState
        (field "players" (list string))
        (field "rounds" (list roundDecoder))


roundDecoder : Decoder Round
roundDecoder =
    map3
        Round
        (field "question" string)
        (field "answers" (list answerDecoder))
        (field "guesses" (list guessDecoder))


answerDecoder : Decoder Answer
answerDecoder =
    map2
        Answer
        (field "player" string)
        (field "answer" string)


guessDecoder : Decoder Guess
guessDecoder =
    map2
        Guess
        (field "player" string)
        (field "answers" (list answerDecoder))

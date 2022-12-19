module Main exposing (main)

import Browser
import Html exposing (Html, b, button, div, h1, h3, input, li, ol, option, pre, text, ul)
import Html.Attributes exposing (class, placeholder, type_)
import Html.Events exposing (onClick, onInput)
import Http exposing (emptyBody)
import Json.Decode exposing (..)
import Json.Encode
import String exposing (fromInt)
import Time



-- MAIN


main =
    Browser.element
        { init = init
        , update = update
        , subscriptions = subscriptions
        , view = view
        }



-- MODEL


type Phase
    = JoinPhase String
    | AnswerPhase
    | GuessPhase
    | ShowResultsPhase


type alias Model =
    { name : String
    , gameName : String
    , phase : Phase
    , gameState : GameState
    , currentAnswer : String
    , currentGuess : List Answer
    , currentRound : Int
    }


init : () -> ( Model, Cmd Msg )
init _ =
    ( Model ""
        ""
        (JoinPhase "Please enter your name and the name of the game you would like to join.")
        { players = [], rounds = [] }
        ""
        []
        1
    , Cmd.none
    )



-- UPDATE


type Msg
    = Tick
    | GetGameData
    | GotGameData (Result Http.Error GameState)
    | JoinGame
    | MakeGame
    | JoinedGame (Result Http.Error ())
    | UpdateName String
    | UpdateGameName String
    | UpdateCurrentAnswer String
    | SubmitAnswer
    | SubmittedAnswer (Result Http.Error ())
    | SubmitGuess
    | SubmittedGuess (Result Http.Error ())
    | ContinueToNextRound


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Tick ->
            case model.phase of
                JoinPhase _ ->
                    ( model, Cmd.none )

                _ ->
                    ( model, getGameData model )

        GetGameData ->
            ( model, getGameData model )

        GotGameData result ->
            case result of
                Ok data ->
                    ( { model | gameState = data }, Cmd.none )

                Err _ ->
                    ( model, Cmd.none )

        MakeGame ->
            ( model, createGame model )

        JoinGame ->
            ( model, joinGame model )

        JoinedGame result ->
            case result of
                Ok data ->
                    ( { model | phase = AnswerPhase }, getGameData model )

                Err x ->
                    ( { model | phase = JoinPhase "Failed to join game, try again." }, Cmd.none )

        UpdateName newName ->
            ( { model | name = newName }, Cmd.none )

        UpdateGameName gameName ->
            ( { model | gameName = gameName }, Cmd.none )

        UpdateCurrentAnswer newAnswer ->
            ( { model | currentAnswer = newAnswer }, Cmd.none )

        SubmitAnswer ->
            if model.currentAnswer == "" then
                ( model, Cmd.none )

            else
                ( model, submitAnswer model )

        SubmittedAnswer result ->
            ( { model | phase = GuessPhase, currentAnswer = "" }, getGameData model )

        SubmitGuess ->
            ( model, submitGuess model )

        SubmittedGuess result ->
            ( { model | phase = ShowResultsPhase }, getGameData model )

        ContinueToNextRound ->
            ( { model | phase = AnswerPhase, currentRound = List.length model.gameState.rounds }, Cmd.none )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    Time.every 1000 (\_ -> Tick)



-- VIEW


view : Model -> Html Msg
view model =
    case model.phase of
        JoinPhase joinMessage ->
            div []
                [ h1 [] [ text "Weighty Inquiry" ]
                , div [] [ text joinMessage ]
                , input [ type_ "text", placeholder "Player Name", Html.Attributes.value model.name, onInput UpdateName ] []
                , input [ type_ "text", placeholder "Game Room Name", Html.Attributes.value model.gameName, onInput UpdateGameName ] []
                , div []
                    [ button [ onClick MakeGame ] [ text "Create Game" ]
                    , button [ onClick JoinGame ] [ text "Join Game" ]
                    ]
                ]

        AnswerPhase ->
            div []
                [ roundNumber model
                , questionAndAnswerForm model
                , playerList model
                ]

        GuessPhase ->
            div []
                [ roundNumber model
                , answersAndGuessForm model
                ]

        ShowResultsPhase ->
            div []
                [ roundNumber model
                , resultsPage model
                ]


playerList : Model -> Html Msg
playerList model =
    div []
        [ h3 [] [ text "Players" ]
        , div [] (List.map (\player -> li [] [ text player ]) model.gameState.players)
        ]


questionAndAnswerForm : Model -> Html Msg
questionAndAnswerForm model =
    div []
        [ h1 [] [ text "Question" ]
        , text
            (case List.reverse model.gameState.rounds of
                [] ->
                    "no questions"

                first :: _ ->
                    first.question
            )
        , h1 [] [ text "Your Answer" ]
        , input [ type_ "text", placeholder "Answer", Html.Attributes.value model.currentAnswer, onInput UpdateCurrentAnswer ] []
        , button [ onClick SubmitAnswer ] [ text "Submit Answer" ]
        ]


answersAndGuessForm : Model -> Html Msg
answersAndGuessForm model =
    div []
        [ h1 [] [ text "Make Guesses" ]
        , case List.reverse model.gameState.rounds of
            [] ->
                text "no questions"

            round :: _ ->
                div []
                    ([ div []
                        [ h3 [] [ text "Question" ]
                        , text round.question
                        ]
                     , h3 [] [ text "Other Answers" ]
                     ]
                        ++ (if List.length round.answers == List.length model.gameState.players then
                                List.map
                                    (\answer ->
                                        if answer.player == model.name then
                                            text ""

                                        else
                                            li [] [ text answer.answer ]
                                    )
                                    round.answers
                                    ++ [ button [ onClick SubmitGuess ] [ text "Submit Guess" ] ]

                            else
                                [ text "waiting for everyone to submit their answers" ]
                           )
                        ++ [ playerList model ]
                    )
        ]


getNth : Int -> List a -> Maybe a
getNth n l =
    case l of
        [] ->
            Nothing

        h :: t ->
            if n == 0 then
                Just h

            else
                getNth (n - 1) t


resultsPage : Model -> Html Msg
resultsPage model =
    div []
        [ h1 [] [ text "Results" ]
        , if List.length model.gameState.rounds == model.currentRound then
            text "waiting for everyone to submit their guesses"

          else
            div []
                (case getNth 1 (List.reverse model.gameState.rounds) of
                    Just round ->
                        List.map (\answer -> li [] [ b [] [ text <| answer.player ++ " - " ], text answer.answer ]) round.answers
                            ++ [ button [ onClick ContinueToNextRound ] [ text "Next Round" ] ]

                    Nothing ->
                        []
                )
        ]


roundNumber : Model -> Html Msg
roundNumber model =
    h1 [] [ text <| "Round " ++ fromInt (List.length model.gameState.rounds) ]


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


serverUrl =
    "http://192.168.1.24:8172/"


createGame : Model -> Cmd Msg
createGame model =
    Http.request
        { method = "PUT"
        , headers = []
        , url = serverUrl ++ "api/v1/game/" ++ model.gameName
        , body = Http.jsonBody (Json.Encode.object [ ( "player", Json.Encode.string model.name ) ])
        , expect = Http.expectWhatever JoinedGame
        , timeout = Nothing
        , tracker = Nothing
        }


joinGame : Model -> Cmd Msg
joinGame model =
    Http.post
        { url = serverUrl ++ "api/v1/game/" ++ model.gameName
        , body = Http.jsonBody (Json.Encode.object [ ( "player", Json.Encode.string model.name ) ])
        , expect = Http.expectWhatever JoinedGame
        }


getGameData : Model -> Cmd Msg
getGameData model =
    Http.get
        { url = serverUrl ++ "api/v1/game/" ++ model.gameName
        , expect = Http.expectJson GotGameData decoder
        }


submitAnswer : Model -> Cmd Msg
submitAnswer model =
    Http.post
        { url = serverUrl ++ "api/v1/game/" ++ model.gameName ++ "/answer"
        , body = Http.jsonBody (Json.Encode.object [ ( "player", Json.Encode.string model.name ), ( "answer", Json.Encode.string model.currentAnswer ) ])
        , expect = Http.expectWhatever SubmittedAnswer
        }


submitGuess : Model -> Cmd Msg
submitGuess model =
    Http.post
        { url = serverUrl ++ "api/v1/game/" ++ model.gameName ++ "/guess"
        , body = Http.jsonBody (Json.Encode.object [ ( "player", Json.Encode.string model.name ), ( "answers", Json.Encode.list encodeAnswer model.currentGuess ) ])
        , expect = Http.expectWhatever SubmittedGuess
        }


encodeAnswer : Answer -> Value
encodeAnswer answer =
    Json.Encode.object [ ( "player", Json.Encode.string answer.player ), ( "answer", Json.Encode.string answer.answer ) ]


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

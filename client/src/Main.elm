module Main exposing (main)

import Browser
import Html exposing (Html, b, button, div, h1, h3, h5, input, li, text, ul)
import Html.Attributes exposing (placeholder, type_)
import Html.Events exposing (onClick, onInput)
import Http
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
    | AddToGuess String String
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
            ( { model | phase = GuessPhase, currentAnswer = "", currentGuess = [] }, getGameData model )

        AddToGuess player answer ->
            let
                newModel =
                    { model | currentGuess = Answer player answer :: model.currentGuess }
            in
            if List.length newModel.currentGuess == List.length model.gameState.players - 1 then
                ( newModel, submitGuess newModel )

            else
                ( newModel, Cmd.none )

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
        , ul []
            (List.map
                (\player ->
                    if player == model.name then
                        div [] []

                    else
                        li [] [ text player ]
                )
                model.gameState.players
            )
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
                                [ ul []
                                    (List.map
                                        (\answer ->
                                            if answer.player == model.name then
                                                text ""

                                            else
                                                li [] [ text answer.answer ]
                                        )
                                        round.answers
                                    )
                                , h3 [] [ text "Who gave this answer?" ]
                                , let
                                    guess =
                                        getNth (List.length model.currentGuess) (List.filter (\ans -> not <| ans.player == model.name) round.answers)
                                  in
                                  case guess of
                                    Just g ->
                                        div []
                                            [ h5 [] [ text g.answer ]
                                            , div []
                                                (List.map
                                                    (\player -> button [ onClick <| AddToGuess player g.answer ] [ text player ])
                                                    (List.filter (\p -> not <| p == model.name || alreadyGuessedPlayer model.currentGuess p) model.gameState.players)
                                                )
                                            ]

                                    Nothing ->
                                        text ""
                                ]

                            else
                                [ text "waiting for everyone to submit their answers" ]
                           )
                        ++ [ playerList model ]
                    )
        ]


alreadyGuessedPlayer : List Answer -> String -> Bool
alreadyGuessedPlayer guesses player =
    case guesses of
        [] ->
            False

        h :: t ->
            if h.player == player then
                True

            else
                alreadyGuessedPlayer t player


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
                        let
                            realAnswers =
                                List.filter (\ans -> not <| ans.player == model.name) round.answers
                        in
                        [ h3 [] [ text "Your Answers" ]
                        , ul [] (List.map (\answer -> li [] [ b [] [ text <| answer.player ++ " - " ], text answer.answer ]) model.currentGuess)
                        , h3 [] [ text "Real Answers" ]
                        , ul [] (List.map (\answer -> li [] [ b [] [ text <| answer.player ++ " - " ], text answer.answer ]) realAnswers)
                        , h3 [] [ text "Points This Round" ]
                        , ul [] <|
                            List.map
                                (\player ->
                                    li []
                                        [ text
                                            (player
                                                ++ " - "
                                                ++ fromInt
                                                    (getTotalScoreForPlayer
                                                        (case getNth 1 (List.reverse model.gameState.rounds) of
                                                            Nothing ->
                                                                []

                                                            Just lastRound ->
                                                                [ lastRound ]
                                                        )
                                                        player
                                                    )
                                            )
                                        ]
                                )
                                model.gameState.players
                        , h3 [] [ text "Total Points" ]
                        , ul [] <| List.map (\player -> li [] [ text (player ++ " - " ++ fromInt (getTotalScoreForPlayer model.gameState.rounds player)) ]) model.gameState.players
                        , button [ onClick ContinueToNextRound ] [ text "Next Round" ]
                        ]

                    Nothing ->
                        []
                )
        ]


getTotalScoreForPlayer : List Round -> String -> Int
getTotalScoreForPlayer rounds player =
    case rounds of
        [] ->
            0

        round :: rest ->
            let
                scoreThisRound =
                    countScore round.answers (getGuess round.guesses player)
            in
            scoreThisRound + getTotalScoreForPlayer rest player


getGuess : List Guess -> String -> List Answer
getGuess guesses player =
    case guesses of
        [] ->
            []

        h :: t ->
            if h.player == player then
                h.answers

            else
                getGuess t player


countScore : List Answer -> List Answer -> Int
countScore ans1 ans2 =
    case ans1 of
        h :: t ->
            if List.member h ans2 then
                1 + countScore t ans2

            else
                countScore t ans2

        _ ->
            0


roundNumber : Model -> Html Msg
roundNumber model =
    h1 [] [ text <| "Round " ++ fromInt model.currentRound ]


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

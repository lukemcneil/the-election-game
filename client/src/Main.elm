module Main exposing (main)
import Browser
import Html exposing (Html, text, button, div, pre)
import Http
import Html.Events exposing (onClick)



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
  = Failure
  | Loading
  | Success String


init : () -> (Model, Cmd Msg)
init _ =
  ( Loading
  , Http.get
      { url = "wrong website"
      , expect = Http.expectString GotText
      }
  )



-- UPDATE


type Msg
  = GotText (Result Http.Error String)
  | GetText


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    GotText result ->
      case result of
        Ok fullText ->
          (Success fullText, Cmd.none)

        Err _ ->
          (Failure, Cmd.none)
    GetText ->
      (Failure, Http.get
      { url = "http://localhost:3000/api/v1/game/5"
      , expect = Http.expectString GotText
      })



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none



-- VIEW


view : Model -> Html Msg
view model =
  case model of
    Failure ->
      div [] [
        button [onClick GetText]
      [ text "get game data" ],
        pre [] [ text "I was unable to load your book."]
      ]

    Loading ->
      text "Loading..."

    Success fullText ->
      div [] [
        button [onClick GetText]
      [ text "get game data" ],
        pre [] [text fullText]
      ]
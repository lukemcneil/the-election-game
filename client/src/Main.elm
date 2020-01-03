module Main exposing (main)

-- Press buttons to increment and decrement a counter.
--
-- Read how it works:
--   https://guide.elm-lang.org/architecture/buttons.html
--

import Browser
import Html exposing (Html, button, div, h1, text)
import Html.Attributes exposing (class)
import Html.Events exposing (onClick)



-- MAIN


main =
    Browser.sandbox { init = init, update = update, view = view }



-- MODEL


type alias Model =
    Int


init : Model
init =
    0



-- UPDATE


type Msg
    = Increment
    | Decrement
    | Reset


update : Msg -> Model -> Model
update msg model =
    case msg of
        Increment ->
            model + 1

        Decrement ->
            model - 1

        Reset ->
            0



-- VIEW


control : Msg -> String -> Html Msg
control msg content =
    button [ onClick msg, class "bg-blue-600 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded" ] [ text content ]


view : Model -> Html Msg
view model =
    div [ class "text-center" ]
        [ h1 [ class "text-4xl" ] [ text "Counter" ]
        , control Decrement "-"
        , div [ class "text-2xl" ] [ text (String.fromInt model) ]
        , control Increment "+"
        , div [ class "" ] [ text ""]
        , control Reset "reset"
        ]

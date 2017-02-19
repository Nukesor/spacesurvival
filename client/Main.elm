module Main exposing (..)

import Html exposing (text)


type alias Model =
    {}


type Msg
    = Noop


main : Program Never {} msg
main =
    Html.program { init = init, update = update, subscriptions = subscriptions, view = view }


init : ( Model, Cmd msg )
init =
    {} ! []


subscriptions : a -> Sub msg
subscriptions model =
    Sub.none


update : msg -> Model -> ( Model, Cmd msg )
update msg model =
    model ! []


view : a -> Html.Html msg
view model =
    text "hello from elm :)"

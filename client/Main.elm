module Main exposing (..)

import Html
import View exposing (..)
import Model exposing (..)
import Animation
import Animations


main : Program Never Model Msg
main =
    Html.program { init = init, update = update, subscriptions = subscriptions, view = View.view }


init : ( Model, Cmd msg )
init =
    let
        initialStyle =
            Animation.style [ Animation.translate (Animation.px 50) (Animation.px 0), Animation.opacity 1.0 ]

        targetStyle =
            Animation.interrupt Animations.wiggle initialStyle
    in
        { style = targetStyle } ! []


subscriptions : Model -> Sub Msg
subscriptions model =
    Animation.subscription Animate [ model.style ]


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Animate frame ->
            { model
                | style = Animation.update frame model.style
            }
                ! []

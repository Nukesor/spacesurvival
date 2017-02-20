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
        targetStyle =
            Animation.interrupt Animations.dialogAppear Animations.dialogAppearStyle
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

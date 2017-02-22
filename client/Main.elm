module Main exposing (..)

import Html
import View exposing (..)
import Model exposing (..)
import Messages exposing (..)
import Animation
import Animations
import Tutorial


main : Program Never Model Msg
main =
    Html.program { init = init, update = update, subscriptions = subscriptions, view = View.view }


init : ( Model, Cmd msg )
init =
    { modals = Tutorial.levels } ! []


subscriptions : Model -> Sub Msg
subscriptions model =
    Animation.subscription AnimateModal <| List.map .animation model.modals


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        AnimateModal frame ->
            let
                animateModal modal =
                    { modal | animation = Animation.update frame modal.animation }
            in
                { model | modals = List.map animateModal model.modals } ! []

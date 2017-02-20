module View exposing (..)

import Animation
import Model exposing (..)
import Templates.Dialog as Dialog
import Html exposing (..)
import Html.Attributes exposing (..)


type Msg
    = Animate Animation.Msg


view : Model -> Html.Html msg
view model =
    div [ class "dialog-container" ]
        [ Dialog.dialog
            (Animation.render model.style)
            [ h2 [] [ text "Welcome to SPACE" ], p [] [ text "Your journey is about to begin." ] ]
        ]

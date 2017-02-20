module View exposing (..)

import Html
import Svg exposing (..)
import Svg.Attributes exposing (..)
import Animation
import Model exposing (..)


type Msg
    = Animate Animation.Msg


view : Model -> Html.Html msg
view model =
    svg [ version "1.1", width "100%", height "100%" ]
        [ circle
            (List.concat
                [ [ cx "50", cy "50", r "50", fill "#efefef" ]
                , Animation.render model.style
                ]
            )
            []
        ]

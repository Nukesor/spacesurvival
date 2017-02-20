module Templates.Dialog exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)


dialog : List (Attribute msg) -> List (Html msg) -> Html msg
dialog attrs contents =
    div (List.append attrs [ class "dialog" ])
        (List.append contents [ button [ type_ "button" ] [ text "OK" ] ])

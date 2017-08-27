module Extra.Array exposing (..)

import Array


any : (a -> Bool) -> Array.Array a -> Bool
any fn array =
    Array.length (Array.filter fn array) > 0

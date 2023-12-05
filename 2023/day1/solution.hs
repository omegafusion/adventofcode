import Data.Char (isDigit)
import Data.List (isPrefixOf, elemIndex)
import Data.List.Split (splitOn)
import Data.Maybe (isJust, maybe)


consumeUntilFirstNumber :: String -> (Integer, String)
consumeUntilFirstNumber list = 
    let (x:xs) = list
        n = findStartingDigit list
    in maybe (consumeUntilFirstNumber xs) (, list) n

findLastNumber :: [Char] -> Integer
findLastNumber list =
    let f :: Integer -> [Char] -> Integer
        f n  []    = n
        f n (x:xs) = 
            let d = findStartingDigit (x:xs)
            in maybe (f n xs) (`f` xs) d
    in f undefined list

findCalibrationValue :: String -> Integer
findCalibrationValue list =
    let (n, rest) = consumeUntilFirstNumber list in
        let m = findLastNumber rest in
            n*10 + m

findCalibrationValues :: [String] -> Integer
findCalibrationValues = sum . map findCalibrationValue . filter nonempty

nonempty :: String -> Bool
nonempty = (/="")

digitNums = [0..9]

digitStringNums = [
    "0",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9"]

digitStringWords = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"]

toDigit :: String -> Maybe Integer
toDigit d = do
    i <- elemIndex d digitStrings
    return ((digitNums ++ digitNums) !! i)

findStartingDigit :: String -> Maybe Integer
findStartingDigit str = 
    case filter (`isPrefixOf` str) digitStrings of
        [] -> Nothing
        [n] -> toDigit n
        ns -> Nothing

digitStrings = digitStringNums ++ digitStringWords

fromFile filename = do
    file <- readFile filename
    return (findCalibrationValues (splitOn "\n" file))

main = fromFile "./input"
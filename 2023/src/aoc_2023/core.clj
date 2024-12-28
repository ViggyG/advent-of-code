(ns aoc-2023.core
  (:gen-class)
  (:require [clojure.string :as str]))

(use 'clojure.java.io)

(def part-one-regex #"\d")
(def part-two-regex #"(?:\d|one|two|three|four|five|six|seven|eight|nine)")

(def string-to-digit
  {"one" "1"
   "two" "2"
   "three" "3"
   "four" "4"
   "five" "5"
   "six" "6"
   "seven" "7"
   "eight" "8"
   "nine" "9"})

(defn convert-to-digit [number]
  (case number
    "one" 1
    "two" 2
    "three" 3
    "four" 4
    "five" 5
    "six" 6
    "seven" 7
    "eight" 8
    "nine" 9
    number))

(defn read-file [filename]
  (with-open [reader (reader filename)]
    (doall (line-seq reader))))

(defn day-1 [filename regex]

  (let [file-contents (read-file filename)
        total! (atom 0)]

    (doseq [line file-contents]
      (let [numbers (re-seq regex line)]

        (when (seq numbers)

          (let [number (Integer/parseInt (str (convert-to-digit (first numbers))
                                              (convert-to-digit (last numbers))))]

            ;;(println line numbers number)
            (swap! total! + number)))))
    (println @total!)))

(defn -main
  [& args]
  (day-1 "./resources/inputs/input1.txt" part-one-regex)
  (day-1 "./resources/inputs/input1.txt" part-two-regex))

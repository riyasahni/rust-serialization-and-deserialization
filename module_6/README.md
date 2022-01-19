# CrustyDB Walkthough 

The goal of this last module 6 of the warm up is a
(walkthrough)[https://en.wikipedia.org/wiki/Software_walkthrough] CrustyDB's
codebase. First, this module gives you some direction on how to go about
exploring the codebase. It asks some questions (Guiding Questions, below) to
test your knowledge as you make progress. These questions are just for you, you
do not need to submit module 6 to gradescope and we you do not need to run any
tests. However, we will evaluate your knowledge of CrustyDB's codebase with a
quiz that will be release on Gradescope (as usual) on this assignment's due
date. Learning how to read somebody else's codebase is important and a skill you
will (likely) need to put in practice many times in the future.

### Guiding Questions

1. Look at the module 1 README to understand crusty-cli. What is the function that actually reads user input? 

2. Place the following components in the order they occur and identify (on a very high level) the purpose of each: Parser, Executor, Optimizer

3. Crusty supports two commands right now: CREATE and something else. What is the other SQL command? 
    -   Hint: you’ll have to go carefully through the above function and follow through on the different paths it can take to answer this question.
    -   Options: value operations, set operations, query operations, select operations

4. Of the above broad type (answer to Q3), what are all the operators crusty supports? This should be clear if you follow the path of functions that led you to the Q3 answer.

5. Find and look at the rust file associated with a scan (yes, we’re not telling
you where it is on purpose). Try to figure out how it interfaces with the
iterators you are going to have to write for milestone 1. 

6. Look at some other operators, like aggregate. Figure out how they get the
data they are processing.
    - What does this tell you about scan vs. other operations (aggregate, filter, join, etc.)
    - Look at the following query plan (it is okay if you don’t totally understand it). What’s one possible order the different nodes of the graph could occur in, based on crusty?

                                 filter on predicate A
							|
							|
						      join 	
							|
							|
 		           filter on predicate Y - - - - - - filter on predicate X
				       |			            |
				       |			            |
		               scan table A	                      scan table B

7. Finally, It’s a good idea to read through all the page and heapstore function definitions, since you’ll be working with them first.


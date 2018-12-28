# Formal Languages and Compilers 

## Spring 2019

An upper-level course for CS majors on formal languages theory and compilers. 

Topics (subject to revision): regular expressions; finite automata; context-free grammars; predictive parsing; LR parsing; abstract syntax; type systems and type-checking; stack layout and activation records; intermediate representations; control-flow graphs; dataflow/liveness analysis; register allocation; garbage collection/runtimes; virtual machines; assemblers. Over the course of the semester, students will implement a full functioning compiler for a small programming language, targeting a bespoke virtual machine. The course requires a significant amount of programming.

|                       |         Details      |
|-----------------------|----------------------|
| **Lecture**           | MWF |
| **Instructor**        | Gordon Stewart (gstewart@ohio.edu) |
| **Office Hours**      | TBD |
| **TA**                | Tim Steinberger |
| **Lab Hours**         | TBD |

## Texbook

TBD

## Course Difficulty

This is a demanding course that requires extensive programming work, in the form of a series of (often increasingly) difficult assignments. Expect to put in at least 10 hours (sometimes much more) per programming assignment.

## Course Structure

The course consists of weekly lectures (MWF), attendance at which is required. To help get you up to speed with the course programming assignments, we'll also hold biweekly lab hours (time TBD). Although attendance at the lab hours is optional, I highly recommend that you attend — at least for the first few weeks of the course. The programming assignments for this course are extensive and time consuming, so be prepared!

In addition to biweekly homework assignments, there will be a midterm exam (Week 7, approximately 15% of your grade) and a final (approximately 25%). The biweekly homeworks (programming assignments) are worth approximately 40%. We'll have weekly quizzes every Tuesday (with probability 1/3), along with bi-weekly offline Blackboard quizzes (total 10%). You get an additional 10% for free, just for signing up for the course.

### Grade Breakdown

| Component               | Percentage |
|-------------------------|-----|
| Programming assignments | 40% |
| Quizzes                 | 10% |
| Midterm exam            | 15% |
| Final exam              | 25% |
| Free points             | 10% |

Blackboard will be used only to report grades and to post lecture notes. Up-to-date information on all other aspects of the course (assignment due dates, etc.) will be posted either on this website or on the Piazza page or both. 

## Schedule

The schedule is subject to revision.

| Week                        | Topic                                 | Reading                        | Assignment |
|-----------------------------|---------------------------------------|--------------------------------|------------|
| Week 1 (14 Jan)             | Intro. to the course, compilers, Rust | [The Rust Book](https://doc.rust-lang.org/book/index.html) 1-3 | Q0 (16 Jan) |
| Week 2 (21 Jan)             | Rust contd. | [The Rust Book](https://doc.rust-lang.org/book/index.html) 4-6, 8 | [PA0: Intro. to Rust](pa/0.md) (25 Jan) |
| Week 3 (28 Jan)             | **NO CLASS (Stewart away)** | | |
| Week 4 (4 Feb)              | Virtual machines, bytecode, assemblers | | Q1 (6 Feb) |
| Week 5 (11 Feb)             | Regular expressions, finite automata, lexers | Appel 2 | [PA1: Assembler](pa/1.md) (13 Feb) |
| Week 6 (18 Feb)             | From regexps to peephole optimizers | | Q2 (20 Feb) |
| Week 7 (25 Feb)             | Runtimes, garbage collection | Appel 13 | [PA2: VM](pa/2.md) (29 Feb) |
| Week 8 (4 Mar)              | Intermediate representations | | Q3 (6 Mar) |
| Week 9 (11 Mar)             | **SPRING BREAK** | |  |
| Week 10 (18 Mar)            | From IR to bytecode | | **PA3: GC+Peephole** (20 Mar) |
| Week 11 (25 Mar)            | Parsing | Appel 3 | Q4 (27 Mar) |
| Week 12 (1 Apr)             | Desugaring, the Grumpy source language | | **PA4: IR** (5 Apr) |
| Week 13 (8 Apr)             | Types and type checking | TAPL 1, 8-9 | Q5 (10 Apr) |
| Week 14 (15 Apr)            | Register allocation | Appel 11 | Q6 (24 Apr) |
| Week 15 (22 Apr)            | Slop | | **PA5: Desugaring** (22 Apr) |
| 29 Apr - 3 May              | **FINAL EXAM PERIOD** | | |

Assignments are due in Blackboard at 11:59pm unless otherwise specified. **Q0**, **Q1**, etc., denote quizzes in Blackboard, generally due on the Wednesdays of weeks without due programming assignments (PAs).

## Homework and Collaboration Policies

### Acceptable Collaboration Matrix

|            | Instructor/GA	| Noninstructor (e.g., Another Student) | 
|------------|----------------|---------------------------------------|
| ***You***  | All collaboration allowed | High-level discussion (of the problems, not your code!) allowed but only after you've started the assignment; must be documented in README as described below |

Unless otherwise noted, homeworks are due Tuesdays by 11:59 p.m. Late homework assignments will be penalized according to the following formula:

* Up to 24 hours late: no deduction, for a max 2 late homeworks per student across the entire semester
* Homeworks later than 24 hours, or from students who have already turned in 2 late homeworks, will receive 0 points.

You may discuss the homework with other students in the class, but only after you've attempted the problems on your own first. If you do discuss the homework problems with others, write the names of the students you spoke with, along with a brief summary of what you discussed, in a README comment at the top of each submission. Example:

```
(* README Gordon Stewart, Assn #1 
I worked with X and Y. We swapped tips regarding the use of pattern-matching in Rust. *)
```

However, **under no circumstances are you permitted to share or directly copy code or other written homework material**, except with course instructors. The code and proofs you turn in must be your own. Remember: homework is there to give **you** practice in the new ideas and techniques covered by the course; it does you no good if you don't engage!

That said, if we find that you have cheated on an assignment in this course, you will immediately:

* Be referred to the Office of Community Standards (which may take disciplinary action against you, possibly expulsion); and
* Flunk the course (receive a final grade of F).

Students in EECS courses such as this one must adhere to the Russ College of Engineering and Technology [Honor Code](https://www.ohio.edu/engineering/academics/academic-integrity.cfm##code), and to the OU [Student Code of Conduct](http://www.ohio.edu/communitystandards/academic/students.cfm). If you haven't read these policies, do so now.

## Students with Disabilities

If you suspect you may need an accommodation based on the impact of a disability, please contact me privately to discuss your specific needs. If you're not yet registered as a student with a disability, contact the Office of Student Accessibility Services first.

## Student Outcomes vs. Course Learning Outcomes

(a) An ability to apply knowledge of computing and mathematics appropriate to the program's student outcomes and to the discipline. Students will be able to:

* Use pattern-matching to decompose and compute on structured data
* Use recursion to write functions that manipulate recursive types such as syntax trees
* Use higher-order functions such as map to manipulate data structures such as lists or trees
* Construct a finite state machine to recognize a given language

(b) An ability to analyze a problem, and identify and define the computing requirements appropriate to its solution. Students will be able to:

* Determine whether a given language is recognizable (e.g., by a RE, DFA, or CFG)
* Identify the recursive functions appropriate for translating programs into a particular intermediate representation, such as static single assignment form

(c) An ability to design, implement, and evaluate a computer-based system, process, component, or program to meet desired needs. Students will be able to:

* Design, implement, and evaluate against a test suite the correctness of, a lexer and parser for a high-level language
* Design, implement, and evaluate against a test suite the correctness of, a type-checker for a high-level language
* Design, implement, and evaluate against a test suite the correctness of, a program transformation mapping expressions to static single assignment form
* Evaluate the purpose, and correctness of, a program transformation mapping code to static single assignment form

(j) An ability to apply mathematical foundations, algorithmic principles, and computer science theory in the modeling and design of computer-based systems in a way that demonstrates comprehension of the tradeoffs involved in design choices. Students will be able to:

* Apply computer science theory to determine whether a given grammar is parseable by recursive descent
* Evaluate the tradeoffs, in terms of asymptotic complexity, of distinct garbage collection algorithms
* Evaluate the tradeoffs in precision vs. computability of static analyses that underlie garbage collection (e.g., for liveness)
* For a given program, use mathematical foundations such as graph theory to evaluate the feasibility of a particular register-allocation strategy

(k) An ability to apply design and development principles in the construction of software systems of varying complexity. Students will be able to:

* Evaluate the tradeoffs, in terms of design complexity, of a modular vs. monolithic compiler implementation
* Design and implement a compiler embodying the modular approach

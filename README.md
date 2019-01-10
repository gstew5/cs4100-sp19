# Formal Languages and Compilers 

## Spring 2019

An upper-level course for CS majors on formal languages theory and compilers. 

Topics (subject to revision): regular expressions; finite automata; context-free grammars; predictive parsing; LR parsing; abstract syntax; type systems and type-checking; stack layout and activation records; intermediate representations; control-flow graphs; dataflow/liveness analysis; register allocation; garbage collection/runtimes; virtual machines; assemblers. Over the course of the semester, students will implement a full functioning compiler for a small programming language, targeting a bespoke virtual machine. The course requires a significant amount of programming.

|                       |         Details      |
|-----------------------|----------------------|
| **Lecture**           | MWF |
| **Instructor**        | Gordon Stewart (gstewart@ohio.edu) |
| **Office Hours**      | M3-5pm, Stocker 355 |
| **TA**                | Tim Steinberger (ts409415@ohio.edu) |
| **Lab Hours**         | T3-5pm, Stocker 307 |

## Texbook

There's no one textbook that covers everything we'll be talking about in this course. Instead, I'll assign readings each week from the following sources (and maybe others):

* [Modern Compiler Implementation in ML, Appel](http://proquest.safaribooksonline.com.proxy.ohiolink.edu:9099/book/software-engineering-and-development/9781107263826), available for free through the Ohio University library
* [The Rust Book](https://doc.rust-lang.org/book/index.html)
* [Crafting Interpreters, Nystrom](http://www.craftinginterpreters.com/contents.html)

## Course Difficulty

This is a demanding course that requires extensive programming work, in the form of a series of (often increasingly) difficult assignments. Expect to put in at least 10 hours (sometimes much more) per programming assignment.

## Course Structure

The course consists of weekly lectures (MWF), attendance at which is required. To help get you up to speed with the course programming assignments, we'll also hold biweekly lab hours (Stocker 307, Tuesdays 3-5pm). Although attendance at the lab hours is optional, I highly recommend that you participate — at least for the first few weeks. The programming assignments for this course are extensive and time consuming, so be prepared!

In addition to biweekly homework assignments, there will be a midterm exam (Week 7, approximately 15% of your grade) and a final (approximately 25%). The biweekly homeworks (programming assignments) are worth approximately 40%. We'll have weekly in-class quizzes every Friday with probability 1/2, along with bi-weekly offline Blackboard quizzes (total 10%). You get an additional 10% for free, just for signing up for the course.

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
| Week 4 (4 Feb)              | Virtual machines, bytecode, assemblers | Crafting Interpreters [14](http://www.craftinginterpreters.com/chunks-of-bytecode.html), [15](http://www.craftinginterpreters.com/a-virtual-machine.html) | Q1 (6 Feb) |
| Week 5 (11 Feb)             | Regular expressions, finite automata, lexers | Appel 2 | [PA1: Assembler](pa/1.md) (13 Feb) |
| Week 6 (18 Feb)             | From regexps to peephole optimizers | [RE Derivatives](re-derivs.pdf), [QRE Derivatives](alur-qre-derivs.pdf) (supplemental) | Q2 (20 Feb) |
| Week 7 (25 Feb)             | Runtimes, garbage collection | Appel 13 | [PA2: VM](pa/2.md) (29 Feb) |
| Week 8 (4 Mar)              | Intermediate representations | [Intermediate Representations](doc/ir.md) | Q3 (6 Mar) |
| Week 9 (11 Mar)             | **SPRING BREAK** | |  |
| Week 10 (18 Mar)            | From IR to bytecode | [Code Generation](doc/codegen.md) | **PA3: GC+Peephole** (20 Mar) |
| Week 11 (25 Mar)            | Parsing | Appel 3 | Q4 (27 Mar) |
| Week 12 (1 Apr)             | Desugaring, the Grumpy source language | [Desugaring](doc/desugar.md), [The Grumpy Source Language](doc/source.md) | **PA4: IR** (5 Apr) |
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

1. Analyze a complex computing problem and to apply principles of computing and other relevant disciplines to identify solutions.

* Students will be able to appraise the tradeoffs, in terms of asymptotic complexity and precision, of distinct algorithms used in compiler construction (e.g., for garbage collection).

2. Design, implement, and evaluate a computing-based solution to meet a given set of computing requirements in the context of the program’s discipline.

* Students will be able to construct a compiler, over the course of a series of course assignments, for a small programming language.

6. Apply computer science theory and software development fundamentals to produce computing-based solutions.

* Students will be able to determine whether a given language is recognizable (e.g., by a regular expression, deterministic finite automaton, or context-free grammar).
* Students will be able to construct a finite state machine to recognize a given language.
* Students will be able to apply computer science theory to determine whether a given grammar is parseable by recursive descent.

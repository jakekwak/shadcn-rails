use dioxus::prelude::*;

const CODE_JS: &str = r#"// Node.js database query using Prisma ORM
const { PrismaClient } = require('@prisma/client');
const prisma = new PrismaClient();

// Create a new user
const user = await prisma.user.create({
  data: {
    name: 'Jane Smith',
    email: 'jane@example.com',
    age: 28,
    profile: {
      create: {
        bio: 'Software developer passionate about clean code'
      }
    }
  }
});

console.log('Created user:', user);

// Alternative using raw SQL with mysql2
const mysql = require('mysql2/promise');
const connection = await mysql.createConnection(config);
const [rows] = await connection.execute(
  'SELECT * FROM users WHERE age > ? AND active = ?',
  [25, true]
);"#;

const CODE_PY: &str = r#"# Python database query using SQLAlchemy ORM
from sqlalchemy import create_engine, Column, Integer, String, Boolean
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker

# Create user
user = User(
    name='Jane Smith',
    email='jane@example.com',
    age=28,
    active=True
)
session.add(user)
session.commit()

# Query users
users = session.query(User).filter(
    User.age > 25,
    User.active == True
).all()

# Alternative using raw SQL with psycopg2
import psycopg2
conn = psycopg2.connect(database="mydb", user="user", password="pass")
cur = conn.cursor()
cur.execute("SELECT * FROM users WHERE age > %s AND active = %s", (25, True))
results = cur.fetchall()"#;

const CODE_GO: &str = r#"package main

import (
    "database/sql"
    "fmt"
    _ "github.com/lib/pq"
)

type User struct {
    ID     int    `db:"id"`
    Name   string `db:"name"`
    Email  string `db:"email"`
    Age    int    `db:"age"`
    Active bool   `db:"active"`
}

func main() {
    db, err := sql.Open("postgres",
        "postgres://user:pass@localhost/dbname?sslmode=disable")
    if err != nil {
        panic(err)
    }
    defer db.Close()

    // Insert user
    _, err = db.Exec(`
        INSERT INTO users (name, email, age, active)
        VALUES ($1, $2, $3, $4)`,
        "Jane Smith", "jane@example.com", 28, true)

    // Query users
    rows, err := db.Query(`
        SELECT id, name, email, age, active
        FROM users WHERE age > $1 AND active = $2`,
        25, true)
    defer rows.Close()

    for rows.Next() {
        var user User
        rows.Scan(&user.ID, &user.Name, &user.Email,
            &user.Age, &user.Active)
        fmt.Printf("User: %+v\n", user)
    }
}"#;

const CODE_RB: &str = r#"# Ruby database query using ActiveRecord ORM
class User < ApplicationRecord
  has_one :profile
  validates :email, presence: true, uniqueness: true
  scope :active_adults, -> { where('age > ? AND active = ?', 25, true) }
end

# Create user with profile
user = User.create!(
  name: 'Jane Smith',
  email: 'jane@example.com',
  age: 28,
  active: true
)

user.create_profile!(
  bio: 'Software developer passionate about clean code'
)

# Query users
active_adults = User.active_adults.includes(:profile)

# Alternative using raw SQL with ActiveRecord
users = User.find_by_sql([
  'SELECT u.*, p.bio FROM users u
   LEFT JOIN profiles p ON u.id = p.user_id
   WHERE u.age > ? AND u.active = ?',
  25, true
])

# Using Arel for complex queries
User.joins(:profile)
    .where(User.arel_table[:age].gt(25))
    .where(active: true)
    .select('users.*, profiles.bio')"#;

struct LangTab {
    id: &'static str,
    label: &'static str,
    filename: &'static str,
    code: &'static str,
}

const TABS: &[LangTab] = &[
    LangTab { id: "javascript", label: "Javascript", filename: "database-query.js", code: CODE_JS },
    LangTab { id: "python", label: "Python", filename: "database_query.py", code: CODE_PY },
    LangTab { id: "go", label: "Go", filename: "database_query.go", code: CODE_GO },
    LangTab { id: "ruby", label: "Ruby", filename: "database_query.rb", code: CODE_RB },
];

#[component]
pub fn Codeexample1() -> Element {
    let mut selected = use_signal(|| 0usize);
    let current = selected();
    let tab = &TABS[current];

    // Re-highlight when tab changes
    use_effect(move || {
        let _ = selected();
        let _ = document::eval(r#"
            setTimeout(() => {
                if (typeof hljs !== 'undefined') {
                    document.querySelectorAll('#codeblock-content code').forEach(el => {
                        el.removeAttribute('data-highlighted');
                        hljs.highlightElement(el);
                    });
                }
            }, 50);
        "#);
    });

    rsx! {
        // Load highlight.js from CDN
        document::Stylesheet { href: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/styles/github-dark.min.css" }
        script { src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/highlight.min.js" }

        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "grid place-items-center gap-10 lg:grid-cols-2 lg:gap-0",
                    // Left: text content
                    div {
                        class: "flex flex-col gap-6 lg:pr-20",
                        span { class: "text-muted-foreground text-lg",
                            "./database-setup.sh"
                        }
                        h2 {
                            class: "text-4xl font-bold tracking-tight md:text-5xl",
                            "YOUR DATA."
                            br {}
                            span { class: "text-muted-foreground", "YOUR QUERIES." }
                        }
                        p { class: "text-muted-foreground md:text-lg",
                            "Connect faster and manipulate your data with database APIs and ORMs that are powerful but also easy to use."
                        }
                        a {
                            href: "https://example.com",
                            class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-11 px-8 bg-primary text-primary-foreground shadow hover:bg-primary/90 w-fit",
                            "Get started"
                            svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M7 7h10v10" }
                                path { d: "M7 17 17 7" }
                            }
                        }
                    }
                    // Right: code block
                    div {
                        class: "flex w-full flex-col gap-1 overflow-hidden",
                        // Tab bar
                        div {
                            class: "inline-flex h-10 w-full items-center justify-center rounded-lg bg-muted p-1 text-muted-foreground",
                            for (i, t) in TABS.iter().enumerate() {
                                button {
                                    key: "{t.id}",
                                    class: if i == current {
                                        "inline-flex items-center justify-center whitespace-nowrap rounded-md px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 bg-background text-foreground shadow-sm flex-1"
                                    } else {
                                        "inline-flex items-center justify-center whitespace-nowrap rounded-md px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 hover:bg-background/50 hover:text-foreground flex-1"
                                    },
                                    onclick: move |_| selected.set(i),
                                    "{t.label}"
                                }
                            }
                        }
                        // Code content
                        div {
                            id: "codeblock-content",
                            class: "rounded-lg border overflow-hidden",
                            // Header with filename + copy
                            div {
                                class: "flex items-center justify-between border-b border-zinc-800 bg-zinc-900 px-4 py-2",
                                span { class: "text-xs text-zinc-400 font-mono", "{tab.filename}" }
                                button {
                                    class: "text-xs text-zinc-400 hover:text-zinc-200 transition-colors",
                                    "Copy"
                                }
                            }
                            // Code body
                            div {
                                class: "max-h-96 overflow-auto",
                                pre {
                                    class: "!m-0 !rounded-none",
                                    code {
                                        class: "language-{tab.id} !text-sm !leading-relaxed",
                                        "{tab.code}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

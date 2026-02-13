# shadcn/ui Dioxus Blocks

React [shadcn/ui](https://ui.shadcn.com) 컴포넌트와 [shadcnblocks](https://shadcnblocks.com) 블록 템플릿을 **Dioxus 0.7 (Rust WASM)** + **Tailwind CSS v4**로 변환한 프로젝트.

## Overview

- **54개 블록 템플릿** — Hero, Feature, Pricing, FAQ, Blog, Footer 등
- **13개 기본 UI 컴포넌트** — Alert, Breadcrumb, Table, Pagination 등
- **23개 스텁/인터랙티브 UI 컴포넌트** — Accordion, Button, Badge, Tabs 등 (`ui.rs`)
- **자동 변환 도구** — `bin/shadcn_to_dioxus` Python CLI (2,500+ LOC)
- **85개 Lucide 아이콘** 인라인 SVG 지원

## Quick Start

```bash
# 의존성
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli

# 빌드 & 실행
cd shadcn_dioxus
dx serve

# Tailwind CSS 빌드 (별도 터미널)
npx @tailwindcss/cli -i input.css -o assets/tailwind.css --watch
```

브라우저에서 `http://localhost:8080` 접속. 홈 페이지에서 54개 블록 링크를 클릭하여 개별 확인.

## Project Structure

```
shadcn_dioxus/
├── Cargo.toml                     # Dioxus 0.7 + router
├── Dioxus.toml                    # dx serve 설정
├── input.css                      # Tailwind v4 입력 (oklch 테마 + prose 오버라이드)
├── assets/
│   └── tailwind.css               # Tailwind CLI 빌드 출력
├── src/
│   ├── main.rs                    # 라우터 (54개 블록 라우트) + Home 인덱스 페이지
│   ├── components/
│   │   ├── mod.rs                 # pub mod + pub use 선언
│   │   ├── ui.rs                  # 핵심 UI 컴포넌트 (23개)
│   │   ├── navigation_menu.rs     # NavigationMenu 컴포넌트
│   │   ├── alert.rs               # Alert 컴포넌트
│   │   ├── table.rs               # Table 컴포넌트
│   │   ├── ...                    # 기타 기본 컴포넌트
│   │   └── *.rs                   # 54개 블록 컴포넌트
│   └── pages/
│       ├── mod.rs
│       └── showcase.rs            # 기본 컴포넌트 데모 페이지
```

## UI Components (`ui.rs`)

`ui.rs`에는 블록 템플릿이 사용하는 핵심 UI 컴포넌트들이 구현되어 있습니다.

### Interactive Components

| 컴포넌트 | 상태 관리 | 설명 |
|----------|-----------|------|
| **Accordion** | `use_context` + `Signal<String>` | `type="single"` 아코디언, 항상 collapsible |
| **AccordionItem** | `use_context_provider` | 자신의 `value`를 자식에 전달 |
| **AccordionTrigger** | 클릭 핸들러 | 열기/닫기 토글 + chevron 회전 애니메이션 |
| **AccordionContent** | 조건부 렌더링 | 닫힌 상태면 렌더링하지 않음 |

### Static Components

| 컴포넌트 | Props | 설명 |
|----------|-------|------|
| Avatar / AvatarImage / AvatarFallback | `class`, `src`, `alt` | 원형 아바타 |
| Badge | `variant` (default/secondary/outline/destructive) | 라벨 배지 |
| Button | `variant` (6종), `size` (4종) | 버튼 |
| Input | `placeholder`, `type` | 텍스트 입력 |
| Label | `for` | 폼 라벨 |
| Separator | `orientation` (horizontal/vertical) | 구분선 |
| Sheet / SheetTrigger / SheetContent | `class` | 사이드 패널 |
| Switch | `checked` | 토글 스위치 |
| Tabs / TabsList / TabsTrigger / TabsContent | `default_value`, `value` | 탭 |
| Tooltip / TooltipTrigger / TooltipContent | `class` | 툴팁 |

### Additional Components (별도 파일)

| 컴포넌트 | 파일 | 설명 |
|----------|------|------|
| Alert / AlertTitle / AlertDescription | `alert.rs` | 상태 메시지 (Default/Destructive) |
| Breadcrumb | `breadcrumb.rs` | 경로 네비게이션 |
| Table | `table.rs` | HTML 테이블 래퍼 |
| NavigationMenu (6개 서브컴포넌트) | `navigation_menu.rs` | 상단 네비게이션 |
| Pagination | `pagination.rs` | 페이지 네비게이션 |
| Carousel | `carousel.rs` | 이미지/콘텐츠 슬라이더 |
| Drawer | `drawer.rs` | 슬라이드 패널 |
| InputOTP | `input_otp.rs` | OTP 코드 입력 |
| Kbd | `kbd.rs` | 키보드 단축키 표시 |
| Spinner | `spinner.rs` | 로딩 인디케이터 |
| Typography | `typography.rs` | h1~h4, p, blockquote |
| ButtonGroup | `button_group.rs` | 버튼 그룹 |
| Resizable | `resizable.rs` | 리사이즈 패널 |

## Block Templates (54개)

모든 블록은 `localhost:8080/{block_name}`으로 접근 가능.

| Category | Blocks |
|----------|--------|
| **Hero** | hero1, hero3, hero7, hero45, hero47, hero115 |
| **Feature** | feature1, feature2, feature13, feature17, feature43, feature51, feature72, feature73, feature166, feature197 |
| **Pricing** | pricing2, pricing4, pricing6 |
| **CTA** | cta10, cta11 |
| **Blog** | blog7, blogpost1 |
| **FAQ** | faq1 |
| **Testimonial** | testimonial10 |
| **Team** | team1 |
| **Stats** | stats8 |
| **Timeline** | timeline9 |
| **Experience** | experience5 |
| **Careers** | careers4 |
| **Auth** | login1, signup1 |
| **Navigation** | navbar1, footer2 |
| **Content** | content1, resource1, service1, services4, changelog1, codeexample1 |
| **About** | about3, casestudies2, casestudy8, compliance1 |
| **Other** | banner1, community1, compare7, contact7, download2, gallery6, integration3, list2, logos8, waitlist1 |

## Converter: `bin/shadcn_to_dioxus`

React shadcn/ui TSX를 Dioxus RSX로 자동 변환하는 Python CLI 도구.

### Usage

```bash
# shadcn 레지스트리에서 변환
python3 bin/shadcn_to_dioxus accordion

# GitHub URL에서 변환
python3 bin/shadcn_to_dioxus https://github.com/shadcnblocks/shadcn-ui-blocks/blob/master/src/block/faq1.tsx

# 로컬 TSX 파일 변환
python3 bin/shadcn_to_dioxus /path/to/component.tsx

# 드라이런 (미리보기)
python3 bin/shadcn_to_dioxus faq1 --dry-run

# 의존성 확인
python3 bin/shadcn_to_dioxus login-03 --list-deps
```

### Conversion Pipeline

```
TSX Input
  │
  ├─ 1. 소스 자동 판별 (레지스트리 / GitHub URL / 로컬 파일)
  ├─ 2. Props 추출 + 기본값 인라이닝
  ├─ 3. TypeScript 타입/인터페이스 제거
  ├─ 4. 모듈 레벨 const 배열 추출
  ├─ 5. JSX → RSX 변환
  │     ├─ className → class
  │     ├─ camelCase attrs → snake_case (viewBox → view_box 등)
  │     ├─ Lucide 아이콘 → 인라인 SVG (85개 지원)
  │     ├─ Button asChild → <a> 태그
  │     ├─ 컴포넌트 props 매핑 (50개 매핑)
  │     └─ 서브 컴포넌트 변환 (56개 매핑)
  ├─ 6. 후처리
  │     ├─ dark:invert 제거
  │     ├─ dark:prose-invert 제거
  │     ├─ fictional-company-logo URL → 실제 로고 URL 교체
  │     └─ collapsible prop 제거
  ├─ 7. .map() → for 루프 변환
  ├─ 8. 배열 props 언롤링 (for 루프 → 정적 요소 확장)
  │     └─ {index} / {idx} → 실제 인덱스 번호
  ├─ 9. use 문 자동 생성
  └─ 10. 출력 파일 생성
           └─ shadcn_dioxus/src/components/{name}.rs
```

### Key Features

- **50개 컴포넌트 매핑** — React shadcn/ui → Dioxus 컴포넌트 자동 변환
- **56개 서브컴포넌트 매핑** — CardHeader, DialogTitle 등 → HTML 태그 + Tailwind 클래스
- **85개 Lucide 아이콘** — `<ArrowRight />` → 인라인 SVG 자동 변환
- **배열 props 언롤링** — `.map()` 루프를 정적 요소로 확장 (데이터 손실 방지)
- **모듈 레벨 const 지원** — 컴포넌트 함수 외부의 `const DATA = [...]` 배열 처리
- **URL 자동 수정** — `fictional-company-logo` → 실제 CDN 로고 URL

## Tailwind CSS v4 Theme

`input.css`에 정의된 oklch 기반 테마:

```css
@theme {
  --color-background: oklch(1 0 0);
  --color-foreground: oklch(0.145 0 0);
  --color-primary: oklch(0.205 0 0);
  --color-primary-foreground: oklch(0.985 0 0);
  --color-secondary: oklch(0.97 0 0);
  --color-muted: oklch(0.97 0 0);
  --color-muted-foreground: oklch(0.556 0 0);
  --color-accent: oklch(0.97 0 0);
  --color-destructive: oklch(0.577 0.245 27.325);
  --color-border: oklch(0.922 0 0);
  --color-input: oklch(0.922 0 0);
  --color-ring: oklch(0.708 0 0);
  /* ... radius tokens ... */
}
```

### Typography Plugin Override

Tailwind `prose` 클래스가 테마 색상을 사용하도록 CSS 변수 오버라이드:

```css
.prose {
  --tw-prose-body: var(--color-foreground);
  --tw-prose-headings: var(--color-foreground);
  --tw-prose-links: var(--color-foreground);
  --tw-prose-quotes: var(--color-foreground);
  --tw-prose-hr: var(--color-border);
  /* ... */
}
```

## Claude Code Skill

`/convert-dioxus` 스킬로 Claude Code에서 직접 사용 가능:

```
/convert-dioxus faq1
/convert-dioxus --dry-run hero3
/convert-dioxus https://github.com/shadcnblocks/shadcn-ui-blocks/blob/master/src/block/pricing6.tsx
```

스킬 정의: `.claude/skills/convert-dioxus/SKILL.md`

## Known Issues & Workarounds

| 이슈 | 원인 | 해결 |
|------|------|------|
| `dark:invert`로 로고 안 보임 | 다크 테마에서 이미 어두운 SVG를 반전 | 컨버터가 자동 제거 |
| `prose` 텍스트가 회색 | Tailwind typography 플러그인 기본 색상 | `input.css`에서 CSS 변수 오버라이드 |
| `fictional-company-logo` 403 | CDN에서 접근 불가 | 실제 로고 URL로 자동 교체 |
| 배열 데이터 truncation | `.map()` 변환 시 첫 항목만 생성 | 배열 언롤링으로 모든 항목 확장 |
| Accordion 콘텐츠 항상 보임 | 스텁 구현 (토글 없음) | `use_context` + `Signal` 인터랙티브 구현 |
| pricing toggle 미동작 | React state를 변환 불가 | `use_signal`로 수동 구현 |

## Tech Stack

| 기술 | 버전 | 용도 |
|------|------|------|
| Rust | 2024 edition | 언어 |
| Dioxus | 0.7.3 | WASM 프레임워크 (web + router) |
| Tailwind CSS | v4 | 스타일링 (oklch, @theme, @plugin) |
| Python | 3.x | 변환 도구 (`bin/shadcn_to_dioxus`) |

## References

- [shadcn/ui](https://ui.shadcn.com) — 원본 React 컴포넌트
- [shadcnblocks](https://shadcnblocks.com) — 블록 템플릿 출처
- [Dioxus](https://dioxuslabs.com) — Rust WASM 프레임워크
- [Tailwind CSS v4](https://tailwindcss.com) — 유틸리티 CSS
- [shadcn-ui-blocks GitHub](https://github.com/shadcnblocks/shadcn-ui-blocks) — 블록 TSX 소스

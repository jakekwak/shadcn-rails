---
name: convert-dioxus
description: Convert React shadcn/ui TSX components and blocks to Dioxus RSX (Rust + Tailwind CSS)
argument-hint: "<source> [--dry-run] [--list-deps]"
allowed-tools: Bash, Read, Write, Edit, Grep, Glob
---

# shadcn_to_dioxus Converter

React shadcn/ui 컴포넌트와 블록을 Dioxus RSX (Rust + Tailwind CSS)로 변환하는 도구.

## 사용법

```bash
python3 bin/shadcn_to_dioxus <source> [options]
```

### 소스 자동 판별

- **shadcn 레지스트리 이름**: `accordion`, `login-03` 등
- **GitHub URL**: `https://github.com/shadcnblocks/shadcn-ui-blocks/blob/master/src/block/about3.tsx`
- **로컬 TSX 파일**: `../shadcn-studio/src/block/about3.tsx`

### 옵션

- `--dry-run`: 파일을 생성하지 않고 변환 결과만 출력
- `--list-deps`: 컴포넌트의 의존성 목록만 출력

## 변환 후 필수 확인 사항

변환 후 반드시 아래 체크리스트를 확인하세요:

### 1. 컴파일 체크
```bash
cd shadcn_dioxus && ~/.cargo/bin/cargo check
```

### 2. RSX 문법 확인
- 모든 속성 뒤에 **쉼표(`,`)** 가 있는지 확인
- 컴포넌트 props에서 `accordion_type: "single",` 처럼 쉼표 필수
- `class: "...",` 끝에도 쉼표 필수

### 3. 콘텐츠 정확성 확인
원본 TSX와 비교하여:
- 배열 데이터의 **모든 항목**이 포함되었는지 (truncation 없음)
- 텍스트 내용이 원본과 일치하는지
- 이미지 URL이 올바른지

### 4. 알려진 문제 자동 수정 (컨버터가 처리)
- `dark:invert` → 제거
- `dark:prose-invert` → 제거
- `fictional-company-logo` URL → 실제 로고 URL로 교체
- `collapsible` prop → 제거 (Accordion은 항상 collapsible)
- `.map()` + 배열 props → for 루프 언롤링 (정적 요소로 확장)
- `{index}` / `{idx}` → 실제 인덱스 번호

## 프로젝트 구조

```
shadcn_dioxus/
├── src/
│   ├── main.rs                    # 라우터 + 데모 페이지
│   ├── components/
│   │   ├── mod.rs                 # pub mod + pub use 선언
│   │   ├── ui.rs                  # 기본 UI 컴포넌트 (Accordion, Button, Badge 등)
│   │   ├── navigation_menu.rs     # NavigationMenu 컴포넌트
│   │   └── *.rs                   # 블록 컴포넌트들
│   └── pages/
│       └── showcase.rs
├── input.css                      # Tailwind v4 + prose 오버라이드
└── assets/tailwind.css
```

## 지원 컴포넌트 매핑

| React (shadcn/ui) | Dioxus RSX | 비고 |
|---|---|---|
| `<Button>` | `Button { variant: "...", ... }` | variant, size props |
| `<Accordion type="single" collapsible>` | `Accordion { accordion_type: "single", ... }` | collapsible 자동, use_signal 토글 |
| `<AccordionItem value="...">` | `AccordionItem { value: "...", ... }` | |
| `<AccordionTrigger>` | `AccordionTrigger { ... }` | 자동 chevron 아이콘 포함 |
| `<AccordionContent>` | `AccordionContent { ... }` | 클릭 시 표시/숨김 |
| `<Tabs>` | `Tabs { default_value: "...", ... }` | |
| `<Badge>` | `Badge { variant: "...", ... }` | |
| `<Avatar>` | `Avatar { ... }` | |
| `<Input>` | `Input { placeholder: "...", ... }` | |
| `<Separator>` | `Separator { orientation: "...", ... }` | |
| `<Button asChild>` | `<a class="...button classes...">` | a 태그로 변환 |
| Lucide 아이콘 | 인라인 SVG | 60+ 아이콘 지원 |

## 예시

```bash
# shadcn 블록 변환
python3 bin/shadcn_to_dioxus faq1

# GitHub에서 직접 변환
python3 bin/shadcn_to_dioxus https://github.com/shadcnblocks/shadcn-ui-blocks/blob/master/src/block/hero3.tsx

# 드라이런으로 결과 미리보기
python3 bin/shadcn_to_dioxus feature51 --dry-run

# 로컬 TSX 파일 변환
python3 bin/shadcn_to_dioxus /tmp/shadcn_blocks_tsx/pricing6.tsx
```

## 변환 결과에 문제가 있을 때

1. 원본 TSX를 `Read`로 직접 읽어서 비교
2. 컨버터 문제는 `bin/shadcn_to_dioxus` 파이썬 코드를 수정
3. 수정 후 반드시 `--dry-run`으로 테스트
4. `cargo check`로 컴파일 확인

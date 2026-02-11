# shadcn_to_rails 변환 도구

React shadcn/ui 컴포넌트(TSX)를 Rails ERB + Helper 형식으로 변환하는 CLI 도구입니다.

## 사용법

### 기본: shadcn 레지스트리에서 가져오기

```bash
# 블록 변환
python3 bin/shadcn_to_rails login-03

# URL 직접 지정
python3 bin/shadcn_to_rails --url https://ui.shadcn.com/r/styles/new-york/sidebar-01.json

# 의존성 확인만
python3 bin/shadcn_to_rails login-03 --list-deps

# 파일 생성 없이 미리보기
python3 bin/shadcn_to_rails login-03 --dry-run
```

### 로컬 TSX 파일 직접 변환

```bash
# 단일 파일
python3 bin/shadcn_to_rails my-component --file path/to/component.tsx

# 여러 파일 (page + sub-components)
python3 bin/shadcn_to_rails my-block --file src/page.tsx --file src/components/form.tsx

# 디렉토리 전체 (내부 .tsx/.jsx 파일 재귀 탐색)
python3 bin/shadcn_to_rails my-block --file src/my-block/

# 출력 경로 지정
python3 bin/shadcn_to_rails my-block --file src/page.tsx --output-dir app/views/custom/
```

### 브라우저에서 확인

변환된 블록은 개발 서버에서 바로 확인할 수 있습니다:

```bash
bin/rails server
# http://localhost:3000/blocks/my-block
```

라우트: `GET /blocks/:block` → `ApplicationController#block` → `layouts/block` 레이아웃 사용

---

## 변환 파이프라인

전체 흐름: **입력(TSX)** → **파싱** → **변환** → **출력(ERB)**

```
┌─────────────┐     ┌──────────────┐     ┌───────────────┐     ┌────────────┐
│  Input       │     │  Parse       │     │  Transform    │     │  Output    │
│              │ ──▸ │              │ ──▸ │               │ ──▸ │            │
│ - Registry   │     │ - Imports    │     │ - Icons       │     │ - ERB file │
│ - Local file │     │ - JSX body   │     │ - Components  │     │ - Helper   │
│              │     │              │     │ - Sub-comps   │     │            │
│              │     │              │     │ - Attributes  │     │            │
│              │     │              │     │ - Expressions │     │            │
│              │     │              │     │ - Post-proc   │     │            │
└─────────────┘     └──────────────┘     └───────────────┘     └────────────┘
```

---

## 1단계: 입력 (Input)

### 레지스트리 모드 (기본)

shadcn 레지스트리 API에서 JSON을 가져옵니다:

```
GET https://ui.shadcn.com/r/styles/new-york/{name}.json
```

응답 구조:
```json
{
  "name": "login-03",
  "type": "registry:block",
  "description": "A login page with...",
  "registryDependencies": ["button", "card", "input", "label"],
  "files": [
    {
      "path": "blocks/login-03/page.tsx",
      "content": "import { ... } ...",
      "type": "registry:page"
    },
    {
      "path": "blocks/login-03/components/login-form.tsx",
      "content": "...",
      "type": "registry:component"
    }
  ]
}
```

### 로컬 파일 모드 (`--file`)

로컬 TSX/JSX 파일을 읽어서 동일한 구조로 변환합니다.

파일 타입 판별 규칙:
- `page.tsx` 또는 `index.tsx` → `registry:page` (메인 페이지)
- 나머지 파일 → `registry:component` (서브 컴포넌트)
- 파일이 하나만 있으면 → `registry:page`

디렉토리를 지정하면 내부의 `.tsx`, `.jsx` 파일을 재귀적으로 수집합니다.

---

## 2단계: 파싱 (Parse)

### 2-1. Import 분석 (`extract_imports`)

TSX의 import 문을 분석하여 아이콘과 컴포넌트를 분류합니다.

```tsx
// 입력
import { Mail, Lock } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardHeader } from "@/components/ui/card"
```

```python
# 결과
{
  "icons": {"Mail", "Lock"},          # Lucide 아이콘
  "components": {                      # shadcn 컴포넌트
    "Button": "@/components/ui/button",
    "Card": "@/components/ui/card",
    "CardHeader": "@/components/ui/card"
  }
}
```

### 2-2. JSX 본문 추출 (`extract_jsx_body`)

컴포넌트 함수에서 `return (...)` 내부의 JSX만 추출합니다.

```tsx
// 입력
export default function LoginForm() {
  const [state, setState] = useState()
  return (
    <div className="flex">     // ← 여기부터
      <Button>Submit</Button>
    </div>                     // ← 여기까지 추출
  )
}
```

괄호 깊이 매칭으로 중첩된 `()`도 올바르게 처리합니다.

---

## 3단계: 변환 (Transform)

`transform_jsx_to_erb()` 함수가 14단계의 변환을 순차적으로 적용합니다.

### 3-1. 정리

| 단계 | 변환 | 예시 |
|------|------|------|
| 1 | import/export 제거 | `import {...}` → (삭제) |
| 2 | `{" "}` → 공백 | JSX 공백 표현 → 일반 공백 |
| 3 | `&apos;` → `'` | HTML 엔티티 정리 |
| 4 | JSX 주석 → ERB 주석 | `{/* comment */}` → `<%# comment %>` |

### 3-2. Lucide 아이콘 변환 (단계 5)

Lucide 아이콘 컴포넌트를 인라인 SVG로 교체합니다.

```tsx
// 입력
<Mail className="size-5" />
```

```erb
<!-- 출력 -->
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
  fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
  stroke-linejoin="round" class="size-5">
  <rect width="20" height="16" x="2" y="4" rx="2"/>
  <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/>
</svg>
```

`LUCIDE_ICONS` 딕셔너리에 미리 정의된 SVG를 사용합니다. 미등록 아이콘은 `<!-- TODO -->` 주석을 남깁니다.

### 3-3. 컴포넌트 변환 (단계 6) — `COMPONENT_MAP`

React shadcn 컴포넌트를 Rails 헬퍼 호출로 변환합니다.

#### 컴포넌트 설정 옵션

```python
COMPONENT_MAP = {
    "Button": {
        "helper": "render_button",  # Rails 헬퍼 함수명
        "text_arg": True,           # 텍스트를 첫 번째 인자로
        "block": True,              # do...end 블록 지원
    },
    "Input": {
        "helper": "render_input",
        "self_closing": True,       # 셀프 클로징 (<Input />)
        "required_props": {         # 필수 prop 매핑
            "name": "id"            # name: prop을 id 값에서 가져옴
        },
    },
    "Label": {
        "helper": "render_label",
        "keyword_only": True,       # 키워드 인자만 사용 (블록/텍스트 인자 없음)
        "required_props": {
            "name": "htmlFor",      # name: prop을 htmlFor 값에서 가져옴
            "label": "text_content"  # label: prop을 자식 텍스트에서 가져옴
        },
    },
    "Card": {
        "helper": "render_card",
        "block": True,
        "inject_class": "w-full",   # 항상 추가되는 클래스
    },
}
```

#### 변환 예시

**셀프 클로징 컴포넌트:**
```tsx
// 입력
<Input type="email" id="email" placeholder="m@example.com" required />
```
```erb
<!-- 출력 -->
<%= render_input(type: "email", id: "email", placeholder: "m@example.com", required: true, name: "email") %>
```

**텍스트 인자 컴포넌트:**
```tsx
// 입력
<Button className="w-full" type="submit">Login</Button>
```
```erb
<!-- 출력 -->
<%= render_button("Login", class: "w-full", type: "submit") %>
```

**블록 컴포넌트:**
```tsx
// 입력
<Card className="overflow-hidden">
  <CardContent>...</CardContent>
</Card>
```
```erb
<!-- 출력 -->
<%= render_card(class: "w-full overflow-hidden") do %>
  <div class="p-6 pt-0">...</div>
<% end %>
```

**키워드 전용 컴포넌트:**
```tsx
// 입력
<Label htmlFor="email">Email</Label>
```
```erb
<!-- 출력 -->
<%= render_label(name: "email", label: "Email") %>
```

### 3-4. 서브 컴포넌트 변환 (단계 7) — `SUB_COMPONENT_MAP`

React 서브 컴포넌트를 HTML 태그 + Tailwind 클래스로 변환합니다.

```python
SUB_COMPONENT_MAP = {
    "CardHeader":      {"tag": "div", "class": "flex flex-col space-y-1.5 p-6"},
    "CardTitle":       {"tag": "h3",  "class": "font-semibold leading-none tracking-tight"},
    "CardDescription": {"tag": "p",   "class": "text-sm text-muted-foreground"},
    "CardContent":     {"tag": "div", "class": "p-6 pt-0"},
    "CardFooter":      {"tag": "div", "class": "flex items-center p-6 pt-0"},
    "DialogHeader":    {"tag": "div", "class": "flex flex-col space-y-1.5 ..."},
    "TabsTrigger":     {"tag": "button", "unwrap": True},  # unwrap: 태그 제거, 자식만 남김
    # ...
}
```

```tsx
// 입력
<CardHeader>
  <CardTitle className="text-2xl">Welcome</CardTitle>
  <CardDescription>Enter your details</CardDescription>
</CardHeader>
```
```erb
<!-- 출력 -->
<div class="flex flex-col space-y-1.5 p-6">
  <h3 class="font-semibold leading-none tracking-tight text-2xl">Welcome</h3>
  <p class="text-sm text-muted-foreground">Enter your details</p>
</div>
```

`className`에 지정된 클래스는 기본 클래스 뒤에 병합됩니다.

### 3-5. 속성 변환 (단계 8)

| JSX | HTML | 설명 |
|-----|------|------|
| `className="..."` | `class="..."` | CSS 클래스 |
| `htmlFor="..."` | `for="..."` | label 연결 |
| `tabIndex={0}` | `tabindex="0"` | 탭 순서 |
| `autoComplete="off"` | `autocomplete="off"` | 자동완성 |
| `onClick={fn}` | (삭제) | 이벤트 핸들러 제거 |
| `{...props}` | (삭제) | 스프레드 제거 |
| `ref={ref}` | (삭제) | React ref 제거 |
| `disabled={true}` | `disabled` | 불리언 속성 |
| `disabled={false}` | (삭제) | false 불리언 제거 |

`className={cn("a", "b")}` 패턴은 `class="a b"`로 변환됩니다.
변수가 포함된 경우 `class="<%= tw("a b", local_assigns[:class]) %>"`로 변환됩니다.

### 3-6. JSX 표현식 변환 (단계 9)

```tsx
// 입력
<span>{userName}</span>
```
```erb
<!-- 출력 -->
<span><%= userName %></span>
```

객체 리터럴이나 함수는 `<%# TODO: ... %>` 주석으로 변환됩니다.

### 3-7. 알 수 없는 컴포넌트 → 파셜 렌더 (단계 10)

`COMPONENT_MAP`이나 `SUB_COMPONENT_MAP`에 없는 PascalCase 컴포넌트는 Rails 파셜 렌더로 변환됩니다.

```tsx
// 입력
<LoginForm />
<CustomWidget className="mt-4">content</CustomWidget>
```
```erb
<!-- 출력 -->
<%= render "blocks/login_03/login_form" %>
<%= render "blocks/login_03/custom_widget" do %>
content
<% end %>
```

### 3-8. 후처리 (단계 11~14)

| 단계 | 처리 | 설명 |
|------|------|------|
| 11 | SVG `size-4` 자동 추가 | `class` 없는 인라인 SVG에 `class="size-4"` 추가 |
| 12 | `text-balance` div → p | 텍스트 전용 `div.text-balance` → `<p>` 태그로 변환 |
| 13 | flex 컨테이너 `min-w-0` | 오버플로 방지를 위해 `tw()` 래핑된 flex에 `min-w-0` 추가 |
| 14 | 빈 줄 정리 | 연속 3줄 이상 공백 → 2줄로 축소 |

---

## 4단계: 출력 (Output)

### 파일 생성 규칙

| 타입 | 출력 경로 | 예시 |
|------|-----------|------|
| `registry:block` (page) | `app/views/blocks/{name}/_{name}.html.erb` | `blocks/login_03/_login_03.html.erb` |
| `registry:block` (component) | `app/views/blocks/{name}/_{sub_name}.html.erb` | `blocks/login_03/_login_form.html.erb` |
| `registry:ui` | `app/views/components/ui/_{name}.html.erb` | `components/ui/_button.html.erb` |
| helper (ui only) | `app/helpers/components/{name}_helper.rb` | `components/button_helper.rb` |

### components.json 자동 업데이트

생성된 파일 정보가 `lib/components.json`에 자동 등록됩니다:

```json
{
  "login-03": {
    "name": "login-03",
    "type": "components:block",
    "files": [
      "app/views/blocks/login_03/_login_03.html.erb",
      "app/views/blocks/login_03/_login_form.html.erb"
    ]
  }
}
```

---

## 알려진 제한 사항

1. **React 런타임 스타일 누락**: React 컴포넌트가 런타임에 주입하는 스타일(예: `text-muted-foreground`)은 TSX 소스에 없어 변환되지 않습니다. 수동 확인이 필요합니다.

2. **Tailwind 클래스 충돌**: `SUB_COMPONENT_MAP`의 기본 클래스와 `className`의 클래스가 충돌할 수 있습니다(예: `p-6 pt-0`과 `p-0`). Tailwind의 마지막 클래스 우선 규칙으로 동작은 하지만 불필요한 클래스가 남을 수 있습니다.

3. **조건부 렌더링**: `{condition && <Component />}` 같은 조건부 렌더링은 `<%= condition %>` 형태로 불완전하게 변환됩니다.

4. **상태 관리**: `useState`, `useEffect` 등 React 훅은 무시됩니다. 인터랙티브 기능은 Stimulus 컨트롤러로 별도 구현해야 합니다.

5. **Lucide 아이콘**: `LUCIDE_ICONS`에 미등록된 아이콘은 `<!-- TODO -->` 주석으로 남습니다. 필요한 아이콘을 스크립트에 추가하거나 수동으로 SVG를 삽입하세요.

---

## 프로젝트 구조

```
shadcn-rails/
├── bin/shadcn_to_rails              # 변환 CLI 도구
├── app/
│   ├── controllers/
│   │   └── application_controller.rb  # block 액션 (블록 프리뷰)
│   ├── helpers/components/            # 컴포넌트 헬퍼 (render_xxx)
│   ├── views/
│   │   ├── blocks/                    # 생성된 블록 파셜
│   │   │   ├── login_01/
│   │   │   ├── login_02/
│   │   │   └── ...
│   │   ├── components/ui/             # UI 컴포넌트 파셜
│   │   └── layouts/
│   │       └── block.html.erb         # 블록 프리뷰 레이아웃 (풀스크린)
│   └── javascript/controllers/ui/     # Stimulus 컨트롤러
├── config/routes.rb                   # /blocks/:block 라우트
└── lib/components.json                # 컴포넌트 레지스트리
```

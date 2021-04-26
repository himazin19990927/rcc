; ModuleID = 'main'
source_filename = "main"

@.str = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

declare i32 @printf(i8*, ...)

define i32 @main() {
entry:
  %local = alloca i32, align 4
  store i32 3, i32* %local, align 4
  %local1 = alloca i32, align 4
  store i32 9, i32* %local1, align 4
  %local2 = alloca i32, align 4
  %0 = load i32, i32* %local, align 4
  %1 = load i32, i32* %local1, align 4
  %2 = mul i32 %0, %1
  %3 = load i32, i32* %local, align 4
  %4 = add i32 %2, %3
  store i32 %4, i32* %local2, align 4
  %5 = load i32, i32* %local2, align 4
  %6 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str, i32 0, i32 0), i32 %5)
  ret i32 0
}
